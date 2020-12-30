extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use syn::parse_quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Expr, FnArg, Ident, ImplItem, ImplItemMethod,
    ItemFn, ItemImpl, ItemStruct, Pat, PatType, ReturnType, Type, Token, Stmt
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

fn is_bytes_type_for_type(ty: &Type) -> bool {
    if let Type::Reference(tr) = ty {
        if let Type::Slice(ts) = &*tr.elem {
            if let Type::Path(path) = &*ts.elem {
                if path.path.get_ident().unwrap().to_string() == "u8" {
                    return true;
                }
            }
        }
    }
    false
}

fn is_plain_type_for_type(ty: &Type) -> bool {
    if let Type::Path(path) = &*ty {
        if path.path.get_ident().unwrap().to_string() == "u8" {
            return true;
        }
    }
    false
}

fn get_arg_name(ty: &PatType) -> Option<(Ident, Span)> {
    if let Pat::Ident(ident) = &*ty.pat {
        return Some((ident.ident.clone(), ident.ident.span()));
    }
    None
}

fn process_async(method: ImplItemMethod, name: String) -> ItemFn {
    let func_name = String::from("rpc_") + &name + "_" + &method.sig.ident.to_string();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let origin_func_name = method.sig.ident;
    // process args
    let mut new_args = method.sig.inputs.clone();
    let mut call_args = Punctuated::<Expr, Token![,]>::new();
    let mut stmt_vec = Vec::new();
    new_args.clear();
    for input in method.sig.inputs.iter() {
        // ignore self.
        if let FnArg::Receiver(_) = input {}
        if let FnArg::Typed(t) = input {
            if is_bytes_type_for_type(&*t.ty) {
                // push two args;
                let name = get_arg_name(&*t).unwrap().0;
                let span = get_arg_name(&*t).unwrap().1;
                let ptr_name_ident = Ident::new(&(name.to_string() + "_ptr"), span);
                let len_name_ident = Ident::new(&(name.to_string() + "_len"), span);

                let ptr_arg: FnArg = parse_quote! (#ptr_name_ident: *const u8);
                new_args.push(ptr_arg);

                let len_arg: FnArg = parse_quote! (#len_name_ident: usize);
                new_args.push(len_arg);

                let call_expr: Expr = parse_quote! (#name);
                call_args.push(call_expr);

                let stmt: Stmt = parse_quote! {
                    let #name = unsafe {
                        core::slice::from_raw_parts(#ptr_name_ident, #len_name_ident)
                    };
                };

                stmt_vec.push(stmt);
            }
            if is_plain_type_for_type(&*t.ty) {
                // println!("is plain.");
                new_args.push(input.clone());
                let name = get_arg_name(&*t).unwrap().0;
                let call_expr: Expr = parse_quote! (#name);
                call_args.push(call_expr);
            }
        }
    }
    // process return
    let mut callback_name = String::from("callback_");
    if let ReturnType::Default = method.sig.output {
        callback_name.push_str("null");
    };

    if let ReturnType::Type(_, ty) = method.sig.output {
        if is_bytes_type_for_type(&*ty) {
            callback_name.push_str("bytes");
        }
    };
    let callback_name_ident = Ident::new(&callback_name, Span::call_site());
    parse_quote! {
        #[no_mangle]
        pub extern fn #func_name_ident (index: usize, #new_args) {
            let mut actor = ACTOR.actor.borrow_mut();
            let runtime = mw_rt::runtime::Runtime::new();
            runtime.spawn(async move {
                #(#stmt_vec)*
                let result = actor.#origin_func_name(#call_args).await;
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

    let expand = quote! {
        #parsed
        #(#v)*
    };
    TokenStream::from(expand)
}
