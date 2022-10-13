use syn::{punctuated::Punctuated, Variant, token::{Comma, Enum}, Error, Fields, spanned::Spanned, Ident};

use quote::{quote, __private::TokenStream};

pub fn derive_enum(ident: &Ident, enum_token: Enum, variants: Punctuated<Variant, Comma>) -> (TokenStream, TokenStream) {
	if variants.len() == 0 {
		return (
			TokenStream::from(Error::new(enum_token.span(), "Can not derive parse from empty enum.").to_compile_error()),
			TokenStream::from(Error::new(enum_token.span(), "Can not derive span from empty enum.").to_compile_error())
		)
	}

	let variants = variants.iter();
	let mut span_variants = Vec::new();
	let variants = variants.map(|v| {
		let variant_ident = &v.ident;
		match &v.fields {
			Fields::Named(fields) => {
				derive_named(variant_ident, fields)
			}
			Fields::Unnamed(fields) => {
				derive_unnamed(ident, variant_ident, fields, &mut span_variants)
			}
			Fields::Unit => {
				Err((
					TokenStream::from(Error::new(variant_ident.span(), "Can not derive parse from a unit variant").to_compile_error()),
					TokenStream::from(Error::new(variant_ident.span(), "Can not derive span from a unit variant").to_compile_error()),
				))
			}
		}
	}).collect::<Result<Vec<_>,_>>();

	let variants = match variants {
		Ok(variants) => variants,
		Err(err) => return err
	};


	let error = format!("Did not find variant for {}", ident);

	(
		quote! {
			let mut error: ::std::option::Option<parsing::ParseError> = ::std::option::Option::None;
			let mut result: ::std::option::Option<Self> = ::std::option::Option::None;
			let mut position = value.position();
			#(#variants)*
			match result {
				::std::option::Option::Some(result) => {
					value.goto(position)?;
					::std::result::Result::Ok(result)
				}
				::std::option::Option::None => match error {
					::std::option::Option::Some(error) => ::std::result::Result::Err(error),
					::std::option::Option::None => ::std::result::Result::Err(parsing::ParseError::not_found(#error, value.position()))
				}
			}
		},
		quote! {
			match self {
				#(#span_variants)*
			}
		}
	)
}

fn derive_named(variant_ident: &Ident, fields: &syn::FieldsNamed) -> Result<TokenStream, (TokenStream, TokenStream)> {
    let fields = fields.named.iter();
    let mut values = Vec::new();
    let mut inputs = Vec::new();
    let mut checks = Vec::new();
    let objects = fields.map(|field| {
		let ty = &field.ty;

		let field_ident = &field.clone().ident.unwrap();
		let value = syn::Ident::new(&format!("inner_{}", field_ident), field.span());
	
		let check = quote! {
			::std::result::Result::Ok(#value)
		};

		let input = quote! {
			#field_ident: #value
		};

		values.push(value.clone());
		inputs.push(input);
		checks.push(check);

		quote! {
			let #value = <#ty as Parse>::parse(&mut enum_value);
		}
	}).collect::<Vec<_>>();
	
	Ok(quote! {
		{
			let mut enum_value = value.clone();
			#(#objects)*
		
			if let (#(#checks),*) = (#(#values),*) {
				value.goto(enum_value.position())?;
				return ::std::result::Result::Ok(Self::#variant_ident{ #(#inputs),* })
			}
		}
	})
}

fn derive_unnamed(ident: &Ident, variant_ident: &Ident, fields: &syn::FieldsUnnamed, span_variants: &mut Vec<TokenStream>) -> Result<TokenStream, (TokenStream, TokenStream)> {
    let fields = fields.unnamed.iter();

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
			let #value = match <#ty as Parse>::parse(&mut enum_value) {
				Err(parsing::ParseError::Error(error_value, position)) => {
					let value = parsing::ParseError::error(&error_value, position);
					error = Some(value.clone());
					Err(value)
				}
				value => value
			};
		}
	}).collect::<Vec<_>>();

	let mut first_value = None;
	let mut last_value = None;
	let span_values = values.iter().enumerate().map(|(index, value)| {
		let mut should_ignore = true;
		if index == 0 {
			first_value = Some(value);
			should_ignore = false;
		}
		if index == values.len() - 1 {
			last_value = Some(value);
			should_ignore = false;
		}
		match should_ignore {
			true => syn::Ident::new(&format!("_{}", value), value.span()),
			false => value.clone()
		}
	}).collect::<Vec<_>>();

	span_variants.push(quote! {
		#ident::#variant_ident(#(#span_values),*) => {
			parsing::charstream::Span::new(#first_value.span().start, #last_value.span().end)
		}
	});

	Ok(quote! {
		{
			let mut enum_value = value.clone();

			#(#objects)*
			if let (#(#tests),*) = (#(#values),*) {
				let cur_value = #ident::#variant_ident(#(#values),*);
				result = match result {
					::std::option::Option::Some(result) => if result.span() > cur_value.span() {
						::std::option::Option::Some(result)
					} else {
						position = enum_value.position();
						::std::option::Option::Some(cur_value)
					},
					::std::option::Option::None => {
						position = enum_value.position();
						::std::option::Option::Some(cur_value)
					}
				}
			}
		}
	})
}