pub mod tokens;
pub mod charstream;

use std::fmt;

use self::charstream::{CharStream, Position};

pub trait Parse {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized;
}

#[derive(Clone)]
pub enum ParseError {
	NotFound(String, Position),
	Error(String, Position),
	EOF(Position)
}

impl ParseError {
	pub fn not_found(cause: &str, position: Position) -> ParseError {
		ParseError::NotFound(cause.to_owned(), position)
	}

	pub fn error(cause: &str, position: Position) -> ParseError {
		ParseError::Error(cause.to_owned(), position)
	}

	pub fn to_error(self, message: &str) -> Self {
		match self {
			Self::NotFound(cause, position) => ParseError::Error(format!("{}: {}", message, cause), position),
			Self::Error(cause, position) => ParseError::Error(cause, position),
			Self::EOF(position) => ParseError::Error(format!("{}: {}", message, "Reached end of file"), position)
		}
	}
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::NotFound(cause, position) => write!(f, "{}:{}:NotFound: '{}'", position.row, position.column, cause),
			Self::Error(cause, position) => write!(f, "{}:{}:Error: '{}'", position.row, position.column, cause),
			Self::EOF(position) => write!(f, "{}:{}: {}", position.row, position.column, "Reached end of file.")
        }
    }
}

#[derive(Debug)]
pub struct Group<D, I> {
	delimiter: D,
	item: I
}

impl<D, I> Parse for Group<D, I> where
	D: tokens::Delimiter,
	I: Parse
{
    fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let start = D::Start::parse(value)?;
		let item = match I::parse(value) {
			Ok(value) => value,
			Err(err) => return Err(err.to_error("Could not parse group"))
		};
		let end = match D::End::parse(value) {
			Ok(value) => value,
			Err(err) => return Err(err.to_error("Could not parse group"))
		};

		let delimiter = D::new(start, end);

		Ok(Self { delimiter, item })
    }
}

#[derive(Debug)]
pub struct List<I, S> {
	items: Vec<(I, Option<S>)>
}

impl<I, S> Parse for List<I, S> where
	I: Parse,
	S: tokens::Token
{
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        let mut items = Vec::new();

		loop {
			let item = match I::parse(value) {
				Ok(value) => value,
				Err(error) => {
					if items.len() > 0 {
						return Err(error);
					}
					break
				}
			};

			let separator = match S::parse(value) {
				Ok(value) => Some(value),
				_ => {
					items.push((item, None));
					break;
				}
			};

			items.push((item, separator));
		}

		Ok(Self { items })
    }
}

#[derive(Debug)]
pub struct StringValue {
	delim: tokens::Quote,
	value: String
}

impl Parse for StringValue {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        let left = <tokens::Quote as tokens::Delimiter>::Start::parse(value)?;
		let mut inner_value = String::new();

		loop {
			match value.peek() {
				Some(value) if *value == '"' => break,
				Some(_) => inner_value.push(value.next().unwrap()),
				_ => break
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
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut identifier = String::new();

		loop {
		    match value.peek() {
				Some(peeked) if peeked.is_alphanumeric() => identifier.push(value.next().unwrap()),
				_ => break
		    }
		}

		if identifier.len() == 0 {
			return Err(ParseError::not_found("Did not find identifier", value.position()));
		}

		Ok(Self { identifier })
    }
}

#[derive(Debug)]
pub struct Number {
	value: String
}

impl Parse for Number {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut number = String::new();
		
		loop {
		    match value.peek() {
		        Some(peeked) if peeked.is_numeric() => number.push(value.next().unwrap()),
				_ => break
		    }
		}

		if number.len() == 0 {
			return Err(ParseError::not_found("Did not find number.", value.position()));
		}

		Ok(Number { value: number })
    }
}

impl<T> Parse for Vec<T> where T: Parse {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut vec = Vec::new();

		let mut item = T::parse(value);
		while item.is_ok() {
			vec.push(item?);
			item = T::parse(value);
		}

		Ok(vec)
	}
}
