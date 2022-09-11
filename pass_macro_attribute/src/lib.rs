use proc_macro::TokenStream;
use syn::{self, spanned::Spanned};
use quote::quote;

#[proc_macro_attribute]
pub fn pass(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let item = syn::parse_macro_input!(item as syn::ItemStruct);


    impl_pass_macro(&attr, &item)
}

fn impl_pass_macro(attr: &[syn::NestedMeta], item: &syn::ItemStruct) -> TokenStream {
    if attr.len() != 2 { return TokenStream::from(syn::Error::new(attr.last().unwrap().span(), "Expected 2 arguments").to_compile_error()); }

    let ident = match &attr[0] {
        syn::NestedMeta::Meta(syn::Meta::Path(value)) if value.get_ident().is_some() => value.get_ident().unwrap(),
        _ => return TokenStream::from(syn::Error::new(attr[0].span(), "Expected identifier").to_compile_error())
    };
    
    let fn_ident = syn::Ident::new(&format!("parse_{}", ident), ident.span());
    
    let format = match &attr[1] {
        syn::NestedMeta::Lit(syn::Lit::Str(value)) => value,
        _ => return TokenStream::from(syn::Error::new(attr[0].span(), "Expected format string").to_compile_error())
    };

    let struct_name = &item.ident;

    let attrs = &item.attrs;
    
    let gen = quote! {
        #(#attrs)*
        struct #struct_name {
            tokens: Vec<<Self as Parsable>::Token>
        }

        impl #struct_name {
            fn #fn_ident(tokens: Vec<<Self as Parsable>::Token>) -> Vec<<Self as Parsable>::Token> {
                todo!()
            }
        }
    };

    gen.into()
}