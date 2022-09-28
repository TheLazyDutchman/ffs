use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Error, DataUnion, spanned::Spanned, DataStruct, Fields, DataEnum};


#[proc_macro_derive(Parsable)]
pub fn parsable_fn(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = &item.ident;
    let generics = &item.generics;

    let function_body = match item.data {
        Data::Struct(DataStruct {struct_token: _, fields, semi_token: _}) => {
            match fields {
                Fields::Named(fields) => {
                    let mut fields = fields.named.iter();

                    let first_field = match fields.next() {
                        Some(value) => value,
                        None => return TokenStream::from(Error::new(ident.span(), "Can not derive parse from an empty struct").to_compile_error())
                    };

                    let first_ident = &first_field.ident;
                    let first_value = syn::Ident::new(&format!("inner_{}", first_ident.clone().unwrap().to_string()), first_ident.span());
                    let first_type = &first_field.ty;

                    let first_definition = quote! {
                        let #first_value = <#first_type as Parse>::parse(value)?;
                    };

                    let definitions = fields.clone().map(|f| {
                        let ident = syn::Ident::new(&format!("inner_{}", f.ident.clone().unwrap().to_string()), f.ident.span());
                        let ty = &f.ty;
                        quote! {
                            let #ident = match <#ty as Parse>::parse(value) {
                                Ok(value) => value,
                                Err(error) => return Err(ParseError::error(&format!("Could not parse {}, because: '{:?}'", stringify!(#ident), error)))
                            };
                        }
                    }).collect::<Vec<_>>();

                    let values = fields.map(|f| {
                        let ident = &f.ident;
                        let value = syn::Ident::new(&format!("inner_{}", f.ident.clone().unwrap().to_string()), f.ident.span());
                        quote! {
                            #ident:#value
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        #first_definition
                        #(#definitions)*

                        ::std::result::Result::Ok(Self { #first_ident: #first_value, #(#values),* })
                    }
                }
                Fields::Unnamed(fields) => {
                    let fields = fields.unnamed.iter();

                    let types = fields.map(|f| &f.ty);
                    let values = types.clone().map(|t| {
                        quote! {
                            <#t as Parse>::parse(value)?
                        }
                    });

                    quote! {
                        let value = Self(#(#values),*);
                        ::std::result::Result::Ok(value)
                    }
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
                        let fields = fields.unnamed.iter();
                        let types = fields.clone().map(|f| &f.ty);

                        let type_objects = types.clone().map(|t| {
                            quote! {
                                <#t as Parse>::parse(&mut value.clone())
                            }
                        });

                        let mut values = Vec::new();
                        let mut tests = Vec::new();
                        let objects = fields.enumerate().map(|(i, field)| {
                            let ty = &field.ty;
                            let value = syn::Ident::new(&format!("value{}", i), ty.span());

                            values.push(value.clone());
                            tests.push(quote! {
                                Ok(#value)
                            });

                            quote! {
                                let #value = match <#ty as Parse>::parse(&mut value.clone()) {
                                    Err(ParseError::Error(value)) => return Err(ParseError::error(&value)),
                                    value => value
                                };
                            }
                        }).collect::<Vec<_>>();

                        Ok(quote! {
                            {
                                #(#objects)*
                                if let (#(#tests),*) = (#(#values),*) {
                                    return ::std::result::Result::Ok(#ident::#variant_ident(#(#values),*));
                                }
                            }
                        })
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

            let error = format!("Did not find variant for {}", ident);

            quote! {
                #(#variants)*
                ::std::result::Result::Err(ParseError::not_found(#error))
            }
        }
        Data::Union(DataUnion {union_token, fields: _}) 
            => return TokenStream::from(Error::new(union_token.span(), "Can not derive Parse from a union type").to_compile_error())
    };

    let gen = quote! {
        impl #generics Parse for #ident #generics {
            fn parse(value: &mut ::std::iter::Peekable<::std::str::Chars>) -> ::std::result::Result<Self, ParseError> {
                #function_body
            }
        }
    };

    gen.into()
}
