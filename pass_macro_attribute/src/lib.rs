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

    let case_tokens: Vec<&syn::Attribute> = attrs.iter().filter(|attr| attr.path.get_ident().unwrap() == "pass").collect();

    if case_tokens.len() == 0 {
        return TokenStream::from(syn::Error::new(item.span(), "Did not find parsing passes, consider adding #[pass(<ident>, <format>)] attribute.").to_compile_error());
    }

    let mut cases = Vec::new();

    for case in case_tokens {
        let attr = case.parse_meta();

        cases.push(match attr {
            Ok(syn::Meta::List(syn::MetaList {path: _, paren_token: _, nested})) => {
                let nested = nested.clone();
                if nested.len() == 0 { continue; }
                
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
        pub enum Token {
            #(#cases),*
        }

        #(#attrs)*
        pub struct #struct_ident {

        }

        impl #struct_ident {
            fn parse_identifier(tokens: Vec<Token>) -> Token {
                todo!()
            }

            fn parse_operator(tokens: Vec<Token>, operator: &'static str) -> Token {
                todo!()
            }
        }

        impl Parsable for #struct_ident {
            type Token = Token;

            fn parse_tokens<'a>(tokens: String) -> Iter<'a, Token> {
                todo!()
            }
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
    if attr.len() != 2 { return TokenStream::from(syn::Error::new(attr.last().expect("Expected 2 arguments.").span(), "Expected 2 arguments").to_compile_error()); }

    let ident = match &attr[0] {
        syn::NestedMeta::Meta(syn::Meta::Path(value)) if value.get_ident().is_some() => value.get_ident().unwrap(),
        _ => return TokenStream::from(syn::Error::new(attr[0].span(), "Expected identifier").to_compile_error())
    };
    
    let fn_ident = syn::Ident::new(&format!("parse_{}", ident.to_string().to_lowercase()), ident.span());
    
    let format = match &attr[1] {
        syn::NestedMeta::Lit(syn::Lit::Str(value)) => value,
        _ => return TokenStream::from(syn::Error::new(attr[1].span(), "Expected format string").to_compile_error())
    }.value();

    let format_tokens = format.split_whitespace();

    let struct_name = &item.ident;
    let attrs = &item.attrs;
    let mut tokens = Vec::new();

    for token in format_tokens {
        if token.len() > 3 && token.starts_with("{") && token.ends_with("}") {
            let ident = &token[1..token.len() - 1];
            let idents:Vec<&str> = ident.split(",").collect();
            let fn_ident = syn::Ident::new(&format!("parse_{}", idents[0]), attr[1].span());

            let idents = &idents[1..];

            let mut args = Vec::new();
            args.push(quote! {
                tokens.clone().map(|s| s.clone()).collect::<Vec<Token>>()
            });

            for ident in idents {
                args.push(quote! {
                    stringify!(ident)
                });
            }

            tokens.push(quote! {
                value.push(Self::#fn_ident(#(#args),*));
            })
        }
    }
    
    let gen = quote! {
        #(#attrs)*
        pub struct #struct_name {
            tokens: Vec<Token>
        }

        impl #struct_name {
            fn #fn_ident(tokens: Vec<Token>) -> Token {
                let mut value = Vec::new();
                let mut tokens = tokens.iter();

                #(#tokens)*

                Token::#ident
            }
        }
    };

    gen.into()
}