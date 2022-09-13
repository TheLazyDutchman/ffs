#![allow(unused)]

use proc_macro::TokenStream;
use syn::{self, spanned::Spanned};
use quote::quote;

#[proc_macro_attribute]
pub fn parsable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let item = syn::parse_macro_input!(item as syn::ItemStruct);

    impl_parsable_macro(&attr, &item)
}

fn impl_parsable_macro(attr: &[syn::NestedMeta], item: &syn::ItemStruct) -> TokenStream {
    if attr.len() > 0 {
        todo!()
    }

    let struct_ident = &item.ident;
    let attrs = &item.attrs;

    let case_tokens: Vec<&syn::Attribute> = attrs.iter().filter(|attr| attr.path.get_ident().unwrap() == "pass")
        .collect();

    let mut cases = Vec::new();

    for case in case_tokens {
        let attr = case.parse_meta();

        cases.push(match attr {
            Ok(syn::Meta::List(syn::MetaList {path: _, paren_token: _, nested})) => {
                let nested = nested.clone();
                let ident = nested.first().unwrap();
                match ident {
                    syn::NestedMeta::Meta(syn::Meta::Path(ident)) if ident.get_ident().is_some() => {
                        ident.get_ident().unwrap().clone()
                    }
                    _ => continue
                }
            }
            _ => continue
        });
    }

    let gen = quote! {
        #[derive(Clone)]
        enum Token {
            #(#cases),*
        }

        #(#attrs)*
        struct #struct_ident {

        }

        impl Parsable for #struct_ident {
            type Token = Token;
        }
    };

    gen.into()
}

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
    
    let fn_ident = syn::Ident::new(&format!("parse_{}", ident.to_string().to_lowercase()), ident.span());
    
    let format = match &attr[1] {
        syn::NestedMeta::Lit(syn::Lit::Str(value)) => value,
        _ => return TokenStream::from(syn::Error::new(attr[0].span(), "Expected format string").to_compile_error())
    }.value();

    let format_tokens = format.split_whitespace();

    let struct_name = &item.ident;
    let attrs = &item.attrs;
    let mut tokens = Vec::new();

    for token in format_tokens {
        if token.len() > 3 && token.starts_with("{") && token.ends_with("}") {
            let ident = &token[1..token.len() - 1];
            let fn_ident = syn::Ident::new(&format!("parse_{}", ident), attr[1].span());
            tokens.push(quote! {
                value.push(Self::#fn_ident(tokens.clone().map(|s| s.clone()).collect()));
            })
        }
    }
    
    let gen = quote! {
        #(#attrs)*
        struct #struct_name {
            tokens: Vec<<Self as Parsable>::Token>
        }

        impl #struct_name {
            fn #fn_ident(tokens: Vec<<Self as Parsable>::Token>) -> Vec<<Self as Parsable>::Token> {
                let mut value = Vec::new();
                let mut tokens = tokens.iter();

                #(#tokens)*

                value
            }
        }
    };

    gen.into()
}