extern crate proc_macro;

use crate::generator::{Args, Method, MethodType, Ret, ValueType};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use std::fs;
use syn::parse_quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Expr, FnArg, Ident, ImplItem, ImplItemMethod,
    ItemFn, ItemImpl, ItemStruct, Pat, PatType, Path, ReturnType, Signature, Stmt, Token, Type,
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

fn get_type(ty: &Type) -> ValueType {
    if let Type::Path(path) = &*ty {
        if path.path.is_ident("u8") {
            return ValueType::U8;
        }
        if path.path.is_ident("u16") {
            return ValueType::U16;
        }
        if path.path.is_ident("u32") {
            return ValueType::U32;
        }
        if path.path.is_ident("u64") {
            return ValueType::U64;
        }
        if path.path.is_ident("i8") {
            return ValueType::I8;
        }
        if path.path.is_ident("i16") {
            return ValueType::I16;
        }
        if path.path.is_ident("i32") {
            return ValueType::I32;
        }
        if path.path.is_ident("i64") {
            return ValueType::I64;
        }
        if path.path.is_ident("usize") {
            return ValueType::Usize;
        }
        if path.path.is_ident("isize") {
            return ValueType::Usize;
        }
        let vec_path: Path = parse_quote!(Vec<u8>);
        let vec_alloc_path: Path = parse_quote!(alloc::vec::Vec<u8>);
        if path.path == vec_path || path.path == vec_alloc_path {
            // println!("asdasdasdas");
            return ValueType::BytesVec;
        }
    }
    if let Type::Reference(tr) = ty {
        if let Type::Slice(ts) = &*tr.elem {
            if let Type::Path(path) = &*ts.elem {
                if path.path.is_ident("u8") {
                    return ValueType::Bytes;
                }
            }
        }
    }
    ValueType::Null
}

fn get_arg_name(ty: &PatType) -> Option<Ident> {
    if let Pat::Ident(ident) = &*ty.pat {
        return Some(ident.ident.clone());
    }
    None
}

fn process_sig(
    sig: &Signature,
    arguments: &mut Vec<Args>,
) -> (
    Punctuated<FnArg, Token![,]>,
    Punctuated<Expr, Token![,]>,
    Vec<Stmt>,
) {
    let mut new_args = sig.inputs.clone();
    let mut call_args = Punctuated::<Expr, Token![,]>::new();
    let mut stmt_vec = Vec::new();

    new_args.clear();

    for input in sig.inputs.iter() {
        // ignore self.
        if let FnArg::Receiver(_) = input {}
        if let FnArg::Typed(t) = input {
            let ty = get_type(&*t.ty);
            if ty.is_bytes() {
                // push two args;
                let name = get_arg_name(&*t).unwrap();
                let ptr_name_ident = Ident::new(&(name.to_string() + "_ptr"), Span::call_site());
                let len_name_ident = Ident::new(&(name.to_string() + "_len"), Span::call_site());

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
                arguments.push(Args {
                    name: name.to_string(),
                    ty: ValueType::Bytes,
                });
            }
            let plain_type = get_type(&*t.ty);
            if plain_type.is_plain() {
                // println!("is plain.");
                new_args.push(input.clone());
                let name = get_arg_name(&*t).unwrap();
                let call_expr: Expr = parse_quote! (#name);
                call_args.push(call_expr);
                arguments.push(Args {
                    name: name.to_string(),
                    ty: plain_type,
                });
            }
        }
    }

    (new_args, call_args, stmt_vec)
}

fn process_async(method: ImplItemMethod, name: String) -> (ItemFn, Method) {
    let func_name = String::from("rpc_") + &name + "_" + &method.sig.ident.to_string();
    let func_name_ident = Ident::new(&func_name, Span::call_site());

    let mut method_json = Method {
        name: func_name,
        ty: MethodType::Async,
        arguments: Vec::new(),
        ret: Ret {
            ty: ValueType::Null,
        },
    };

    let (new_args, call_args, stmt_vec) = process_sig(&method.sig, &mut method_json.arguments);

    let origin_func_name = method.sig.ident;

    // process return
    let mut callback_name = String::from("callback_");
    if let ReturnType::Default = method.sig.output {
        callback_name.push_str("null");
        method_json.ret.ty = ValueType::Null;
    };

    if let ReturnType::Type(_, ty) = method.sig.output {
        let plain_type = get_type(&*ty);
        callback_name.push_str(plain_type.to_json_type());
        method_json.ret.ty = plain_type;
    };

    let callback_name_ident = Ident::new(&callback_name, Span::call_site());
    (
        parse_quote! {
            #[no_mangle]
            pub extern fn #func_name_ident (index: usize, #new_args) {
                let mut actor = ACTOR.actor.borrow_mut();
                let runtime = mw_rt::runtime::Runtime::new();
                runtime.spawn(async move {
                    #(#stmt_vec)*
                    let result = actor.#origin_func_name(#call_args).await;
                    drop(actor);
                    mw_rt::rpc::#callback_name_ident(index, result);
                });
            }
        },
        method_json,
    )
}

fn is_plain_type_for_return(ret: &ReturnType) -> bool {
    if let ReturnType::Default = ret {
        return true;
    }
    if let ReturnType::Type(_, t) = ret {
        let ty = get_type(t);
        return ty.is_plain();
    }
    false
}

fn process_plain(method: ImplItemMethod, name: String) -> (ItemFn, Method) {
    let func_name = String::from("rpc_") + &name + "_" + &method.sig.ident.to_string();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let mut method_json = Method {
        name: func_name,
        ty: MethodType::Async,
        arguments: Vec::new(),
        ret: Ret {
            ty: ValueType::Null,
        },
    };

    let (new_args, call_args, stmt_vec) = process_sig(&method.sig, &mut method_json.arguments);

    let origin_func_name = method.sig.ident;

    let return_type = method.sig.output;

    (
        parse_quote! {
            #[no_mangle]
            pub extern fn #func_name_ident (#new_args) #return_type {
                let mut actor = ACTOR.actor.borrow_mut();
                #(#stmt_vec)*
                actor.#origin_func_name(#call_args)
            }
        },
        method_json,
    )
}

fn process_func(func: ImplItem, struct_name: String) -> Option<(ItemFn, Method)> {
    if let ImplItem::Method(m) = func.clone() {
        if let Some(_) = m.sig.asyncness {
            return Some(process_async(m, struct_name));
        }
        if m.sig.asyncness.is_none() && is_plain_type_for_return(&m.sig.output) {
            // process plain type.
            return Some(process_plain(m, struct_name));
        }
    }
    None
}

pub fn expose(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut parsed = parse_macro_input!(input as ItemImpl);
    let mut v = Vec::new();

    let self_name = if let Type::Path(p) = &*parsed.self_ty {
        p.path.get_ident().unwrap().to_string().to_case(Case::Snake)
    } else {
        panic!("compile error");
    };

    let mut method_json_array = Vec::new();

    for func in &mut parsed.items {
        // to deal option
        if let Some((f, mj)) = process_func(func.clone(), self_name.clone()) {
            v.push(f);
            method_json_array.push(mj);
        }
    }

    fs::create_dir_all(String::from("target/abi/")).unwrap();
    fs::write(
        String::from("target/abi/") + &self_name + ".json",
        serde_json::to_string_pretty(&method_json_array).unwrap(),
    )
    .unwrap();

    let expand = quote! {
        #parsed
        #(#v)*
    };
    TokenStream::from(expand)
}
