extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemFn};

/// Declare an async main
#[proc_macro_attribute]
pub fn async_main(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemFn);
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn _entry() {
            #parsed
            let runtime = mw_rt::runtime::Runtime::new();

            runtime.spawn(async move {
                main().await
            });
        }
    };
    TokenStream::from(expanded)
}

/// Declare an main
#[proc_macro_attribute]
pub fn main(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemFn);
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn _entry() {
            #parsed
            main();
        }
    };
    TokenStream::from(expanded)
}
