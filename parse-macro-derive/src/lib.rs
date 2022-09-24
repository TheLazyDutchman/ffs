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
                    let fields = fields.named.iter();

                    let idents = fields.clone().map(|f| &f.ident).flatten().collect::<Vec<_>>();

                    let definitions = fields.map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            let #ident = <#ty as parsing::Parse>::parse(value)?;
                        }
                    }).collect::<Vec<_>>();

                    quote! {
                        #(#definitions)*

                        ::std::result::Result::Ok(Self { #(#idents),* })
                    }
                }
                Fields::Unnamed(fields) => {
                    let fields = fields.unnamed.iter();

                    let types = fields.map(|f| &f.ty);
                    let values = types.clone().map(|t| {
                        quote! {
                            <#t as parsing::Parse>::parse(value)?
                        }
                    });

                    quote! {
                        let value = Self(#(#values),*);
                        ::std::result::Result::Ok(value)
                    }
                }
                Fields::Unit => return TokenStream::from(Error::new(ident.span(), "Can not derive parsing::Parse from a unit struct").to_compile_error())
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
                        let types = fields.map(|f| &f.ty);

                        let type_objects = types.clone().map(|t| {
                            quote! {
                                <#t as parsing::Parse>::parse(value)
                            }
                        });

                        let mut values = Vec::new();
                        for (i, ty) in types.enumerate() {
                            values.push(syn::Ident::new(&format!("value{}", i), ty.span()));
                        }

                        let tests = values.iter().map(|v| quote! {
                            Ok(#v)
                        });

                        Ok(quote! {
                            if let (#(#tests),*) = (#(#type_objects),*) {
                                return ::std::result::Result::Ok(#ident::#variant_ident(#(#values),*));
                            }
                        })
                    }
                    Fields::Unit => {
                        Err(TokenStream::from(Error::new(variant_ident.span(), "Can not derive parsing::Parse from a unit variant").to_compile_error()))
                    }
                }
            }).collect::<Result<Vec<_>,_>>();

            let variants = match variants {
                Ok(variants) => variants,
                Err(err) => return err
            };

            let error = format!("Can not parse {}", ident);

            quote! {
                #(#variants)*
                ::std::result::Result::Err(parsing::ParseError::new(#error))
            }
        }
        Data::Union(DataUnion {union_token, fields: _}) 
            => return TokenStream::from(Error::new(union_token.span(), "Can not derive parsing::Parse from a union type").to_compile_error())
    };

    let gen = quote! {
        impl #generics parsing::Parse for #ident #generics {
            fn parse(value: &str) -> ::std::result::Result<Self, parsing::ParseError> {
                #function_body
            }
        }
    };

    gen.into()
}
