use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro]
pub fn main(input: TokenStream) -> TokenStream {
    let fn = parse_macro_input!(input as ItemFn);
    qoute::qoute! {
        
    }
    input
}