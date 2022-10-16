use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Error, DataUnion, spanned::Spanned, DataStruct, DataEnum, GenericParam};

mod derive_struct;
mod derive_enum;

use derive_struct::derive_struct;
use derive_enum::derive_enum;

#[proc_macro_derive(Parsable, attributes(whitespace, value))]
pub fn parsable_fn(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = &item.ident;
    let generics = &item.generics;

    let (parse_body, span_body) = match item.data {
        Data::Struct(DataStruct {struct_token: _, fields, semi_token: _}) => {
            derive_struct(ident, fields)
        }
        Data::Enum(DataEnum { enum_token, brace_token: _, variants}) => {
            derive_enum(ident, enum_token, variants)
        }
        Data::Union(DataUnion {union_token, fields: _}) 
            => return TokenStream::from(Error::new(union_token.span(), "Can not derive parse from a union type").to_compile_error())
    };

    let generics_end = if generics.params.is_empty() {
        quote! {}
    } else {
        let params = generics.params.iter();
        let params = params.map(|param| match param {
            GenericParam::Const(value) => {
                let ident = &value.ident;
                quote! {
                    #ident
                }
            }
            param => quote! {#param}
        });
        quote! {
            <#(#params),*>
        }
    };

    let gen = quote! {
        impl #generics Parse for #ident #generics_end {
            fn parse(value: &mut parsing::charstream::CharStream) -> ::std::result::Result<Self, parsing::ParseError> {
                #parse_body
            }

            fn span(&self) -> parsing::charstream::Span {
                #span_body
            }
        }
    };

    gen.into()
}
