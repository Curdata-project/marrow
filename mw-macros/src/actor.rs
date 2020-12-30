extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use convert_case::{Case, Casing};
use syn::parse_quote;
use crate::generator::{Method, MethodType, ValueType, Ret, Args};
use syn::{
    parse_macro_input, punctuated::Punctuated, Expr, FnArg, Ident, ImplItem, ImplItemMethod,
    ItemFn, ItemImpl, ItemStruct, Pat, PatType, ReturnType, Type, Token, Stmt
};
use std::fs;

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

fn is_plain_type_for_type(ty: &Type) -> (bool, &str) {
    if let Type::Path(path) = &*ty {
        if path.path.get_ident().unwrap().to_string() == "u8" {
            return (true, "u8");
        }
        if path.path.get_ident().unwrap().to_string() == "u16" {
            return (true, "u16");
        }
        if path.path.get_ident().unwrap().to_string() == "u32" {
            return (true, "u32");
        }
        if path.path.get_ident().unwrap().to_string() == "u64" {
            return (true, "u64");
        }
        if path.path.get_ident().unwrap().to_string() == "i8" {
            return (true, "i8");
        }
        if path.path.get_ident().unwrap().to_string() == "i16" {
            return (true, "i16");
        }
        if path.path.get_ident().unwrap().to_string() == "i32" {
            return (true, "i32");
        }
        if path.path.get_ident().unwrap().to_string() == "i64" {
            return (true, "i64");
        }
        if path.path.get_ident().unwrap().to_string() == "usize" {
            return (true, "usize");
        }
        if path.path.get_ident().unwrap().to_string() == "isize" {
            return (true, "isize");
        }
    }
    (false, "")
}

fn get_arg_name(ty: &PatType) -> Option<Ident> {
    if let Pat::Ident(ident) = &*ty.pat {
        return Some(ident.ident.clone());
    }
    None
}

fn process_async(method: ImplItemMethod, name: String) -> (ItemFn, Method) {
    let func_name = String::from("rpc_") + &name + "_" + &method.sig.ident.to_string();
    let func_name_ident = Ident::new(&func_name, Span::call_site());
    let origin_func_name = method.sig.ident;
    // process args
    let mut new_args = method.sig.inputs.clone();
    let mut call_args = Punctuated::<Expr, Token![,]>::new();
    let mut stmt_vec = Vec::new();
    new_args.clear();

    let mut method_json = Method {
        name: func_name,
        ty: MethodType::Async,
        arguments: Vec::new(),
        ret: Ret {
            ty: ValueType::Null,
        }
    };

    for input in method.sig.inputs.iter() {
        // ignore self.
        if let FnArg::Receiver(_) = input {}
        if let FnArg::Typed(t) = input {
            if is_bytes_type_for_type(&*t.ty) {
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
                method_json.arguments.push(Args { name: name.to_string(), ty: ValueType::Bytes });
            }
            let plain_type = is_plain_type_for_type(&*t.ty);
            if plain_type.0 {
                // println!("is plain.");
                new_args.push(input.clone());
                let name = get_arg_name(&*t).unwrap();
                let call_expr: Expr = parse_quote! (#name);
                call_args.push(call_expr);
                method_json.arguments.push(Args { name: name.to_string(), ty: ValueType::from(plain_type.1) });
            }
        }
    }
    // process return
    let mut callback_name = String::from("callback_");
    if let ReturnType::Default = method.sig.output {
        callback_name.push_str("null");
        method_json.ret.ty = ValueType::Null;
    };

    if let ReturnType::Type(_, ty) = method.sig.output.clone() {
        if is_bytes_type_for_type(&*ty) {
            callback_name.push_str("bytes");
            method_json.ret.ty = ValueType::Bytes;
        }
    };

    if let ReturnType::Type(_, ty) = method.sig.output {
        let plain_type = is_plain_type_for_type(&*ty);
        if plain_type.0 {
            callback_name.push_str(plain_type.1);
            method_json.ret.ty = ValueType::from(plain_type.1);
        }
    };

    // println!("{}", serde_json::to_string_pretty(&method_json).unwrap());

    let callback_name_ident = Ident::new(&callback_name, Span::call_site());
    (parse_quote! {
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
    }, method_json)
}

fn process_func(func: ImplItem, struct_name: String) -> Option<(ItemFn, Method)> {
    if let ImplItem::Method(m) = func.clone() {
        if let Some(_) = m.sig.asyncness {
            return Some(process_async(m, struct_name));
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

    fs::write(String::from("target/abi") + &self_name + ".json", serde_json::to_string_pretty(&method_json_array).unwrap()).unwrap();

    let expand = quote! {
        #parsed
        #(#v)*
    };
    TokenStream::from(expand)
}
