extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use syn::parse_quote;
use syn::{
    parse_macro_input, Ident, ImplItem, ImplItemMethod, ItemFn, ItemImpl, ItemStruct, ReturnType,Type
};

/// Declare an main
pub fn actor(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);
    let main_code = quote! {
        async fn main() {
            ACTOR.actor.borrow_mut().init().await;
        }
    };

    let async_main = crate::async_main(_arg, TokenStream::from(main_code));
    let async_main_parsed = parse_macro_input!(async_main as ItemFn);

    let struct_name = parsed.ident.clone();
    let wrapper_struct_name =
        Ident::new(&(struct_name.to_string() + "Wrapper"), struct_name.span());
    let expanded = quote! {
        #parsed

        struct #wrapper_struct_name {
            actor: RefCell<#struct_name>,
            lastest_bytes_length: RefCell<usize>,
        }

        unsafe impl Sync for #wrapper_struct_name {}

        #[macro_use]
        extern crate lazy_static;
        lazy_static! {
            static ref ACTOR: #wrapper_struct_name = #wrapper_struct_name {
                actor: RefCell::new(#struct_name::new()),
                lastest_bytes_length: RefCell::new(0),
            };
        }

        #[no_mangle]
        pub extern "C" fn __get_lastest_bytes_length() -> usize {
            *ACTOR.lastest_bytes_length.borrow()
        }

        #async_main_parsed
    };
    TokenStream::from(expanded)
}

fn process_async(method: ImplItemMethod, name: String) -> ItemFn {
    // let method_clone = method.clone();
    let func_name = String::from("rpc_") + &name + "_" + &method.sig.ident.to_string();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let origin_func_name = method.sig.ident;
    // process args
    // process return
    let mut callback_name = String::from("callback_");
    if let ReturnType::Default = method.sig.output {
        callback_name.push_str("null");
    };

    if let ReturnType::Type(_, ty) = method.sig.output {
        if let Type::Reference(tr) = *ty {
            if let Type::Slice(ts) = *tr.elem {
                if let Type::Path(path) = *ts.elem {
                    if path.path.get_ident().unwrap().to_string() == "u8" {
                        callback_name.push_str("bytes");
                    }
                }
            }
        }
    };
    let callback_name_ident = Ident::new(&callback_name, Span::call_site());
    parse_quote! {
        #[no_mangle]
        pub extern fn #func_name_ident (index: usize) {
            let mut actor = ACTOR.actor.borrow_mut();
            let runtime = mw_rt::runtime::Runtime::new();
            runtime.spawn(async move {
                let result = actor.#origin_func_name().await;
                mw_rt::rpc::#callback_name_ident(index, result);
            });
        }
    }
}

fn process_func(func: ImplItem, struct_name: String) -> Option<ItemFn> {
    if let ImplItem::Method(m) = func.clone() {
        if let Some(_) = m.sig.asyncness {
            return Some(process_async(m, struct_name));
        }
    }
    None
}

pub fn expose(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let mut parsed = parse_macro_input!(input as ItemImpl);
    // println!("{:?}", parsed);
    let mut v = Vec::new();

    for func in &mut parsed.items {
        let self_ty = *parsed.self_ty.clone();
        let mut hasher = DefaultHasher::new();
        self_ty.hash(&mut hasher);
        let f = process_func(func.clone(), format!("{}", hasher.finish()));
        v.push(f)
    }

    let mut expand = quote! {
        #parsed
    };
    // let mut expanded = TokenStream::from(expand);
    for i in v {
        i.to_tokens(&mut expand);
    }
    TokenStream::from(expand)
}
