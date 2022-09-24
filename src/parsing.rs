pub mod tokens;

use parse_macro_derive::Parsable;

pub trait Parse {
	fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized;
}

#[derive(Debug)]
pub struct ParseError {
	cause: String
}

impl ParseError {
	pub fn new(cause: &str) -> ParseError {
		ParseError { cause: cause.to_owned() }
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
    fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized {
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
    fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized {
        let items = Vec::new();

		Ok(Self { items })
    }
}

#[derive(Parsable)]
pub struct StringValue {
	begin: tokens::Quote,
	value: Identifier,
	end: tokens::Quote
}

pub struct Identifier {}

impl Parse for Identifier {
    fn parse(value: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

pub struct Number;

impl Parse for Number {
    fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized {
        todo!()
    }
}

impl<T> Parse for Vec<T> where T: Parse {
	fn parse(value: &str) -> Result<Self, ParseError> {
		let mut vec = Vec::new();

		let mut item = T::parse(value);
		while item.is_ok() {
			vec.push(item?);
			item = T::parse(value);
		}

		Ok(vec)
	}
}
