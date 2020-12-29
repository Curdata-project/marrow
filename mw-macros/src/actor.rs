extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemStruct, Ident, ItemFn};

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
    let wrapper_struct_name = Ident::new(&(struct_name.to_string() + "Wrapper"), struct_name.span());
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