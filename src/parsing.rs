pub mod tokens;

use std::{str::Chars, iter::Peekable};

use parse_macro_derive::Parsable;

pub trait Parse {
	fn parse(value: &mut Peekable<Chars>) -> Result<Self, ParseError> where Self: Sized;
}

#[derive(Debug)]
pub enum ParseError {
	NotFound(String),
	Error(String)
}

impl ParseError {
	pub fn not_found(cause: &str) -> ParseError {
		ParseError::NotFound(cause.to_owned())
	}

	pub fn error(cause: &str) -> ParseError {
		ParseError::Error(cause.to_owned())
	}
}

pub struct Group<D, I> {
	delimiter: D,
	item: I
}

impl<D, I> Parse for Group<D, I> where
	D: tokens::Delimiter,
	I: Parse
{
    fn parse(value: &mut Peekable<Chars<'_>>) -> Result<Self, ParseError> where Self: Sized {
		let start = D::Start::parse(value)?;
		let item = I::parse(value)?;
		let end = D::End::parse(value)?;

		let delimiter = D::new(start, end);

		Ok(Self { delimiter, item })
    }
}

pub struct List<I, S> {
	items: Vec<(I, Option<S>)>
}

impl<I, S> Parse for List<I, S> where
	I: Parse,
	S: tokens::Token
{
    fn parse(value: &mut Peekable<Chars<'_>>) -> Result<Self, ParseError> where Self: Sized {
        let items = Vec::new();

		Ok(Self { items })
    }
}

pub struct StringValue {
	delim: tokens::Quote,
	value: String
}

impl Parse for StringValue {
    fn parse(value: &mut Peekable<Chars>) -> Result<Self, ParseError> where Self: Sized {
        let left = <tokens::Quote as tokens::Delimiter>::Start::parse(value)?;
		let mut inner_value = String::new();

		loop {
			match value.peek() {
				Some(value) if *value == '"' => break,
				Some(_) => inner_value.push(value.next().unwrap()),
				_ => return Err(ParseError::error("Could not find end of string"))
			}
		}

		let right = <tokens::Quote as tokens::Delimiter>::End::parse(value)?;

		Ok(Self { delim: tokens::Delimiter::new(left, right), value: inner_value})
    }
}

pub struct Identifier {
	identifier: String
}

impl Parse for Identifier {
    fn parse(value: &mut Peekable<Chars<'_>>) -> Result<Self, ParseError> {
		let mut identifier = String::new();

		loop {
		    match value.peek() {
				Some(peeked) if peeked.is_alphanumeric() => identifier.push(value.next().unwrap()),
				_ => break
		    }
		}

		if identifier.len() == 0 {
			return Err(ParseError::not_found("Did not find identifier"));
		}

		Ok(Self { identifier })
    }
}

pub struct Number {
	value: String
}

impl Parse for Number {
    fn parse(value: &mut Peekable<Chars<'_>>) -> Result<Self, ParseError> where Self: Sized {
		let mut number = String::new();
		
		loop {
		    match value.peek() {
		        Some(peeked) if peeked.is_numeric() => number.push(value.next().unwrap()),
				_ => break
		    }
		}

		if number.len() == 0 {
			return Err(ParseError::not_found("Did not find number."));
		}

		Ok(Number { value: number })
    }
}

impl<T> Parse for Vec<T> where T: Parse {
	fn parse(value: &mut Peekable<Chars<'_>>) -> Result<Self, ParseError> {
		let mut vec = Vec::new();

		let mut item = T::parse(value);
		while item.is_ok() {
			vec.push(item?);
			item = T::parse(value);
		}

		Ok(vec)
	}
}
