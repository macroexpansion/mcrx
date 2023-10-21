use proc_macro::TokenStream;
use syn::{parse, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();

    TokenStream::new()
}
