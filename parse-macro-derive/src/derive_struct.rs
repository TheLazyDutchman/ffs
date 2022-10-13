use syn::{Ident, Fields, Error, spanned::Spanned, Meta};
use quote::{quote, __private::TokenStream};

pub fn derive_struct(ident: &Ident, fields: Fields) -> (TokenStream, TokenStream) {
	match fields {
		Fields::Named(fields) => derive_named(ident, fields),
		Fields::Unnamed(fields) => derive_unnamed(fields),
		Fields::Unit => return (
			TokenStream::from(Error::new(ident.span(), "Can not derive parse from a unit struct").to_compile_error()),
			TokenStream::from(Error::new(ident.span(), "Can not derive span from a unit struct").to_compile_error())
		)
	}
}

fn derive_named(ident: &Ident, fields: syn::FieldsNamed) -> (TokenStream, TokenStream) {
	let first_field = fields.named[0].clone().ident.unwrap();
	let last_field = fields.named[fields.named.len() - 1].clone().ident.unwrap();

	let span_body = quote! {
		parsing::charstream::Span::new(self.#first_field.span().start, self.#last_field.span().end)
	};

	let mut fields = fields.named.iter();

	let first_field = match fields.next() {
		Some(value) => value,
		None => return (
			TokenStream::from(Error::new(ident.span(), "Can not derive parse from an empty struct").to_compile_error()),
			TokenStream::from(Error::new(ident.span(), "Can not derive span from an empty struct").to_compile_error())
		)
	};

	let first_ident = &first_field.ident;
	let first_value = syn::Ident::new(&format!("inner_{}", first_ident.clone().unwrap().to_string()), first_ident.span());
	let first_type = &first_field.ty;

	let whitespace = &first_field.attrs.iter().filter(|attr| {
		if let Some(ident) = attr.path.get_ident() {
			ident == "whitespace"
		} else {
			false
		}
	}).collect::<Vec<_>>();

	if whitespace.len() > 1 {
		return (
			TokenStream::from(Error::new(first_field.span(), "Can not have two types of whitespace at the same time").to_compile_error()),
			TokenStream::default()
		)
	}

	let first_definition = if whitespace.len() == 1 {
		let attr = &whitespace[0].parse_meta();
		match attr {
			Ok(Meta::Path(path)) => return (
				TokenStream::from(Error::new(path.span(), "Expected a value in the whitespace attr").to_compile_error()),
				TokenStream::default()
			),
			Ok(Meta::List(list)) => {
				let value = &list.nested;
				quote! {
					let #first_value = {
						let mut __inner_buffer = value.clone();
						__inner_buffer.set_whitespace(parsing::charstream::WhitespaceType::#value);

						let inner = <#first_type as Parse>::parse(&mut __inner_buffer)?;
						value.goto(__inner_buffer.position())?;
						inner
					};
				}
			},
			Ok(Meta::NameValue(value)) => return (
				TokenStream::from(Error::new(value.span(), "Did not expected name value pair in whitespace attribute.").to_compile_error()),
				TokenStream::default()
			),
			Err(error) => return (
				TokenStream::from(error.to_compile_error()),
				TokenStream::default()
			)
		}
	} else {
		quote! {
			let #first_value = <#first_type as Parse>::parse(value)?;
		}
	};

	let definitions = fields.clone().map(|f| {
		let ident = syn::Ident::new(&format!("inner_{}", f.ident.clone().unwrap().to_string()), f.ident.span());
		let ty = &f.ty;

		let attr = f.attrs.iter().filter(|attr| {
			if let Some(ident) = attr.path.get_ident() {
				ident == "whitespace"
			} else {
				false
			}
		}).collect::<Vec<_>>();

		if attr.len() > 2 {
			return TokenStream::from(Error::new(f.span(), "Can not have two types of whitespace at the same time").to_compile_error())
		}

		if attr.len() == 1 {
			let attr = &attr[0].parse_meta();
		match attr {
			Ok(Meta::Path(path)) => return TokenStream::from(Error::new(path.span(), "Expected a value in the whitespace attr").to_compile_error()),
			Ok(Meta::List(list)) => {
				let value = &list.nested;
				quote! {
					let #ident = {
						let mut __inner_buffer = value.clone();
						__inner_buffer.set_whitespace(parsing::charstream::WhitespaceType::#value);

						let inner = <#ty as Parse>::parse(&mut __inner_buffer)?;
						value.goto(__inner_buffer.position())?;
						inner
					};
				}
			},
			Ok(Meta::NameValue(value)) => return TokenStream::from(Error::new(value.span(), "Did not expected name value pair in whitespace attribute.").to_compile_error()),
			Err(error) => return TokenStream::from(error.to_compile_error())
		}
		} else {
			quote! {
				let #ident = match <#ty as Parse>::parse(value) {
					Ok(value) => value,
					Err(error) => return Err(parsing::ParseError::error(&format!("Could not parse {}, because: '{:?}'", stringify!(#ident), error), value.position()))
				};
			}
		}
	}).collect::<Vec<_>>();

	let values = fields.map(|f| {
		let ident = &f.ident;
		let value = syn::Ident::new(&format!("inner_{}", f.ident.clone().unwrap().to_string()), f.ident.span());
		quote! {
			#ident:#value
		}
	}).collect::<Vec<_>>();

	(
		quote! {
			#first_definition
			#(#definitions)*

			::std::result::Result::Ok(Self { #first_ident: #first_value, #(#values),* })
		},
		span_body
	)
}

fn derive_unnamed(fields: syn::FieldsUnnamed) -> (TokenStream, TokenStream) {
    let last_field = syn::Index::from(fields.unnamed.len() - 1);
	let fields = fields.unnamed.iter();

	let types = fields.map(|f| &f.ty);
	let values = types.clone().map(|t| {
		quote! {
			<#t as Parse>::parse(value)?
		}
	});

	(
		quote! {
			let result = Self(#(#values),*);
			::std::result::Result::Ok(result)
		},
		quote! {
			parsing::charstream::Span::new(self.0.span().start, self.#last_field.span().end)
		}
	)
}