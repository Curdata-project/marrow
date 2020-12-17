extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemFn);
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn _entry() {
            #parsed
            let runtime = wstd::runtime::Runtime::new();
            runtime.spawn(async move {
                main().await
            });
        }
    };
    TokenStream::from(expanded)
    // expanded
}