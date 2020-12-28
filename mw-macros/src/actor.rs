extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemStruct};

/// Declare an main
pub fn actor(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);
    let expanded = quote! {
        #parsed

        #[macro_use]
        extern crate lazy_static;
        lazy_static! {
            pub static ref ACTOR: #parsed.ident::new();
        }
    };
    TokenStream::from(expanded)
}