pub mod tokens;
pub mod charstream;

use std::fmt;

use self::{charstream::{CharStream, Position, WhitespaceType, Span}, tokens::Delimiter};

pub trait Parse {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized;
	fn span(&self, value: &CharStream) -> Span;
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

	fn span(&self, value: &CharStream) -> Span {
		self.delimiter.span(value)
	}
}

#[derive(Debug)]
pub struct List<I, S> {
	items: Vec<(I, Option<S>)>,
	span: Span
}

impl<I, S> Parse for List<I, S> where
	I: Parse,
	S: tokens::Token
{
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        let mut items = Vec::new();
		let start = value.position();

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

		let end = value.position();

		Ok(Self { items, span: Span::new(start, end) })
    }

	fn span(&self, _: &CharStream) -> Span {
		self.span.clone()
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

		let mut string_value = value.clone();

		let mut position = string_value.position();

		string_value.set_whitespace(WhitespaceType::KeepAll);
		loop {
			match string_value.next() {
				Some(value) if value != '"' => {
					inner_value.push(value);
					position = string_value.position();
				}
				_ => break
			}
		}

		value.goto(position)?;
		
		let right = <tokens::Quote as tokens::Delimiter>::End::parse(value)?;

		Ok(Self { delim: tokens::Delimiter::new(left, right), value: inner_value})
    }

	fn span(&self, value: &CharStream) -> Span {
		self.delim.span(value)
	}
}

#[derive(Clone)]
pub struct Identifier {
	identifier: String,
	span: Span
}

impl Parse for Identifier {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut identifier = String::new();
		let start = value.position();

		let mut ident_value = value.clone();
		match ident_value.next() {
			Some(chr) if chr.is_alphabetic() => {
				let mut position = ident_value.position();
				identifier.push(chr);

				ident_value.set_whitespace(WhitespaceType::KeepAll);

				loop {
					match ident_value.next() {
						Some(value) if value.is_alphanumeric() => {
							identifier.push(value);
							position = ident_value.position();
						}
						_ => break
					}
				}

				value.goto(position)?;
			}
			_ => return Err(ParseError::not_found("Did not find identifier", ident_value.position()))
		}

		let end = value.position();

		Ok(Self { identifier , span: Span::new(start, end)})
    }

	fn span(&self, _: &CharStream) -> Span {
		self.span.clone()
	}
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier({}, from {})", self.identifier, self.span)
    }
}

pub struct Number {
	value: String,
	span: Span
}

impl Parse for Number {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut number = String::new();
		let start = value.position();
		
		let mut num_value = value.clone();
		match num_value.next() {
			Some(chr) if chr.is_numeric() => {
				let mut position = num_value.position();
				number.push(chr);

				num_value.set_whitespace(WhitespaceType::KeepAll);

				loop {
					match num_value.next() {
						Some(value) if value.is_numeric() => {
							number.push(value);
							position = num_value.position();
						}
						_ => break
					}
				}

				value.goto(position)?;
			}
			_ => return Err(ParseError::not_found("Did not find number", num_value.position()))
		}

		let end = value.position();

		Ok(Number { value: number, span: Span::new(start, end)})
    }

	fn span(&self, _: &CharStream) -> Span {
		self.span.clone()
	}
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number({}, from {})", self.value, self.span)
    }
}

impl<T> Parse for Vec<T> where T: Parse + fmt::Debug {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
		let mut vec = Vec::new();

		let mut item = T::parse(value);
		while item.is_ok() {
			vec.push(item?);
			item = T::parse(value);
		}

		println!("end of list: {:?}, cur vec: {:?}", item, vec);

		Ok(vec)
	}

	fn span(&self, value: &CharStream) -> Span {
		if self.len() == 0 {
			return Span::empty(value);
		}

		Span::new(self.first().unwrap().span(value).start, self.last().unwrap().span(value).start)
	}
}

impl<T, const N: usize> Parse for [T; N] where T: Parse + fmt::Debug {
    fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        let mut result = Vec::new();

		for _ in 0..N {
			result.push(T::parse(value)?);
		}

		match <[T; N]>::try_from(result) {
			Ok(result) => Ok(result),
			Err(error) => Err(ParseError::error(&format!("Could not create slice from parsed values. \nvalues where: {:?}", error), value.position()))
		}
    }

	fn span(&self, value: &CharStream) -> Span {
		Span::new(self[0].span(value).start, self[N - 1].span(value).end)
	}
}

//TODO: see if this can be more general
impl<A, B> Parse for (A, B) where
	A: Parse,
	B: Parse
{
    fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        Ok((
			A::parse(value)?,
			B::parse(value)?
		))
    }
	
	fn span(&self, value: &CharStream) -> Span {
		Span::new(self.0.span(value).start, self.1.span(value).end)
	}
}

impl<A, B, C> Parse for (A, B, C) where
	A: Parse,
	B: Parse,
	C: Parse
{
    fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
        Ok((
			A::parse(value)?,
			B::parse(value)?,
			C::parse(value)?
		))
    }

	fn span(&self, value: &CharStream) -> Span {
		Span::new(self.0.span(value).start, self.2.span(value).end)
	}
}