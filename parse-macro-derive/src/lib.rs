use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Error, DataUnion, spanned::Spanned, DataStruct, Fields, Field, DataEnum};


#[proc_macro_derive(Parsable)]
pub fn parsable_fn(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = &item.ident;
    let generics = &item.generics;

    let function_body = match item.data {
        Data::Struct(DataStruct {struct_token: _, fields, semi_token: _}) => {
            match fields {
                Fields::Named(fields) => {
                    let fields = fields.named.iter();

                    let idents = fields.clone().map(|f| &f.ident).flatten().collect::<Vec<_>>();

                    let definitions = fields.map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            let #ident = <#ty as Parse>::parse(value)?;
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        #(#definitions)*

                        ::std::result::Result::Ok(Self { #(#idents),* })
                    }
                }
                Fields::Unnamed(fields) => {
                    todo!()
                }
                Fields::Unit => return TokenStream::from(Error::new(ident.span(), "Can not derive Parse from a unit struct").to_compile_error())
            }
        }
        Data::Enum(DataEnum { enum_token: _, brace_token: _, variants}) => {
            let variants = variants.iter();
            let variants = variants.map(|v| {
                let variant_ident = &v.ident;
                match &v.fields {
                    Fields::Named(fields) => {
                        todo!()
                    }
                    Fields::Unnamed(fields) => {
                        todo!()
                    }
                    Fields::Unit => {
                        Err(TokenStream::from(Error::new(variant_ident.span(), "Can not derive Parse from a unit variant").to_compile_error()))
                    }
                }
            }).collect::<Result<Vec<_>,_>>();

            let variants = match variants {
                Ok(variants) => variants,
                Err(err) => return err
            };

            quote! {
                #(#variants)*
                Err(parsing::ParseError::new(&format!("Can not parse {}", stringify!(#ident))))
            }
        }
        Data::Union(DataUnion {union_token, fields: _}) 
            => return TokenStream::from(Error::new(union_token.span(), "Can not derive Parse from a union type").to_compile_error())
    };

    let gen = quote! {
        impl #generics Parse for #ident #generics {
            fn parse(value: &str) -> Result<Self, parsing::ParseError> {
                #function_body
            }
        }
    };

    gen.into()
}