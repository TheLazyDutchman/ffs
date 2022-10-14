use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Error, DataUnion, spanned::Spanned, DataStruct, DataEnum};

mod derive_struct;
mod derive_enum;

use derive_struct::derive_struct;
use derive_enum::derive_enum;

#[proc_macro_derive(Parsable, attributes(whitespace))]
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

    let gen = quote! {
        impl #generics Parse for #ident #generics {
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
