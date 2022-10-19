use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, parse_macro_input, Data, Error, spanned::Spanned, DataStruct, DataEnum, Ident, Fields, Field, Attribute, Meta, MetaList, Index};


#[proc_macro_derive(Parsable, attributes(whitespace, value))]
pub fn parsable_fn(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    match &item.data {
        Data::Struct(value) => derive_struct(&item.ident, value),
        Data::Enum(value) => derive_enum(&item.ident, value),
        Data::Union(_) => TokenStream::from(Error::new(item.span(), "Can not derive Parse from a union type.").to_compile_error())
    }
}

fn derive_struct(ident: &Ident, value: &DataStruct) -> TokenStream {
    let fields = value.fields.iter().collect::<Vec<_>>();
    let definitions = derive_fields(fields.clone());
    let parse_result = match &value.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| &field.ident);
            let inner_fields = fields.clone().enumerate().map(|(i, field)| inner_ident(&field, i));
            quote! {
                ::std::result::Result::Ok(Self {
                    #(#fields: #inner_fields),*
                })
            }
        }
        Fields::Unnamed(fields) => {
            let inner_fields = fields.unnamed.iter().enumerate().map(|(i, _)| inner_ident(&None, i));
            quote! {
                ::std::result::Result::Ok(Self(#(#inner_fields),*))
            }
        }
        Fields::Unit => return TokenStream::from(Error::new(ident.span(), "Can not derive trait Parse for a unit struct.").to_compile_error())
    };
    let first_ident = get_ident(&fields.first().unwrap().ident, 0);
    let last_ident = get_ident(&fields.last().unwrap().ident, fields.len() - 1);
    quote! {
        impl Parse for #ident {
            fn parse(value: &mut parsing::charstream::CharStream) -> ::std::result::Result<Self, parsing::ParseError> {
                #(#definitions)*
                #parse_result
            }

            fn span(&self) -> parsing::charstream::Span {
                parsing::charstream::Span::new(self.#first_ident.span().start, self.#last_ident.span().end)
            }
        }
    }.into()
}

fn derive_enum(ident: &Ident, value: &DataEnum) -> TokenStream {
    let variants = value.variants.iter().map(|variant| {
        let ident = Ident::new(&format!("__parse_{}", variant.ident.to_string().to_lowercase()), variant.span());
        (&variant.ident, ident, &variant.fields, &variant.attrs)
    });
    let variant_functions = match variants.clone()
        .map(|(field_ident, func_ident, fields, attrs)| derive_variant_function(field_ident, func_ident, fields, attrs))
        .collect::<Result<Vec<_>,_>>() {
            Ok(value) => value,
            Err(error) => return error
        };

    let parse_variants = variants.clone().map(|(_, func_ident, _, _)| {
        quote! {
            let mut __value = value.clone();
            match Self::#func_ident(&mut __value) {
                ::std::result::Result::Ok(inner) => {
                    value.goto(__value.position())?;
                    options.push(inner);
                }
                ::std::result::Result::Err(err) => error = ::std::option::Option::Some(err)
            }
        }
    });

    let span_variants = variants.map(|(variant_ident, _, fields, _)| {
        let fields = fields.iter().collect::<Vec<_>>();
        let definitions = fields.iter().enumerate().map(|(i, field)| match &field.ident {
            Some(ident) => {
                let inner = inner_ident(&field.ident, i);
                quote! {
                    #ident: #inner
                }
            }
            None => {
                let ident = inner_ident(&field.ident, i);
                quote! {#ident}
            }
        });

        let first = inner_ident(&fields.first().unwrap().ident, 0);
        let last = inner_ident(&fields.last().unwrap().ident, fields.len() - 1);

        quote! {
            Self::#variant_ident(#(#definitions),*) => 
                parsing::charstream::Span::new(#first.span().start, #last.span().end),
        }
    });

    quote! {
        impl #ident {
            #(#variant_functions)*
        }

        impl Parse for #ident {
            fn parse(value: &mut parsing::charstream::CharStream) -> ::std::result::Result<Self, parsing::ParseError> {
                let mut options = Vec::new();
                let mut error = None;
                #(#parse_variants)*
                options.sort_by(|a, b| a.span().partial_cmp(&b.span()).unwrap());
                options.first().and_then(|option| Some(option.clone())).ok_or(error.unwrap())
            }

            fn span(&self) -> parsing::charstream::Span {
                match self {
                    #(#span_variants)*
                }
            }
        }
    }.into()
}

fn derive_variant_function(field_ident: &Ident, func_ident: Ident, fields: &Fields, _attrs: &[Attribute]) -> Result<quote::__private::TokenStream, TokenStream> {
    let definitions = derive_fields(fields.iter().collect());
    let parse_result = match fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| &field.ident);
            let inner_fields = fields.clone().enumerate().map(|(i, field)| inner_ident(&field, i));
            quote! {
                ::std::result::Result::Ok(Self::#field_ident {
                    #(#fields: #inner_fields),*
                })
            }
        }
        Fields::Unnamed(fields) => {
            let inner_fields = fields.unnamed.iter().enumerate().map(|(i, _)| inner_ident(&None, i));
            quote! {
                ::std::result::Result::Ok(Self::#field_ident(#(#inner_fields),*))
            }
        }
        Fields::Unit => return Err(TokenStream::from(Error::new(field_ident.span(), "Can not derive trait Parse for a unit variant.").to_compile_error()))
    };
    Ok(quote! {
        fn #func_ident(value: &mut parsing::charstream::CharStream) -> ::std::result::Result<Self, parsing::ParseError> {
            #(#definitions)*
            #parse_result
        }
    })
}

fn derive_fields(fields: Vec<&Field>) -> Vec<quote::__private::TokenStream> {
    let fields = fields.iter().enumerate().map(|(i, field)| {
        let field = field.clone().clone();
        (inner_ident(&field.ident, i), field.ty, field.attrs)
    });
    fields.map(|(ident, ty, attrs)| {
        let whitespace_attr = get_attr(&attrs, "whitespace");
        let value_attr = get_attr(&attrs, "value");

        let value = match whitespace_attr {
            Some(attr) => {
                let whitespace = attr.nested;
                quote! {
                    {
                        let mut __whitespace_value = value.clone();
                        __whitespace_value.set_whitespace(parsing::charstream::WhitespaceType::#whitespace);

                        let inner =  <#ty>::parse(&mut __whitespace_value);
                        value.goto(__whitespace_value.position())?;
                        inner
                    }
                }
            }
            None => quote! { 
                <#ty>::parse(value)
            }
        };
        let value = match value_attr {
            Some(attr) => {
                let mut values = attr.nested.iter().map(|meta| quote! { 
                    ::std::result::Result::Ok(inner) if inner == #meta => inner
                }).collect::<Vec<_>>();
                values.push(quote! { 
                    ::std::result::Result::Ok(inner) => return ::std::result::Result::Err(parsing::ParseError::error("Value was not one of the expected values.", value.position()))
                });
                values.push(quote! { 
                    ::std::result::Result::Err(error) => return ::std::result::Result::Err(error)
                });
                quote! {
                    match #value {
                        #(#values),*
                    }
                }
            }
            None => quote! {
                #value?
            }
        };
        quote! {
            let #ident = #value;
        }
    }).collect::<Vec<_>>()
}

fn get_attr(attrs: &Vec<Attribute>, value: &str) -> Option<MetaList> {
    attrs.iter().find_map(|attr| match attr.path.get_ident() {
        Some(ident) if ident == value => {
            match attr.parse_meta() {
                Ok(Meta::List(list)) => Some(list),
                _ => None
            }
        }
        _ => None
    })
}

fn inner_ident(ident: &Option<Ident>, index: usize) -> Ident {
    let ident = get_ident(ident, index);
    Ident::new(&format!("__inner_{}", ident), ident.span())
}

fn get_ident(ident: &Option<Ident>, index: usize) -> quote::__private::TokenStream {
    match ident {
        Some(ident) => <Ident as ToTokens>::to_token_stream(&ident),
        None => <Index as ToTokens>::to_token_stream(&Index::from(index))
    }
}