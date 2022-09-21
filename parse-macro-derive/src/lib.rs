use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(Parse)]
pub fn parse_fn(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = &item.ident;

    let gen = quote! {
        impl Parse for #ident {

        }
    };

    gen.into()
}