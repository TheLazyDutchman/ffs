use std::{error::Error, fmt::Display, iter::Peekable, slice::Iter};

use self::token::{Token, TokenStream};

pub mod token;

#[derive(Debug)]
pub struct ParserError {
	msg: String,
}

impl Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.msg)
	}
}

impl Error for ParserError {
}

impl ParserError {
	pub fn new(msg: String) -> Self {
		Self { msg }
	}

	pub fn eof() -> Self {
		Self { msg: "Unexpected end of input".to_owned() }
	}
}

pub trait Parsable {
	type Token;
}

pub trait AST {
	fn parse(value: String) -> Result<Self, ParserError> 
			where 
			Self: Sized {
		let mut tokens = TokenStream::from(value);
		tokens
			.keywords(Self::keywords())
			.operators(Self::operators());
		
		if Self::ignore_whitespace() {
			tokens.remove_whitespace();
		}

		let mut tokens = tokens.iter().peekable();

		Self::parse_tokens(&mut tokens)
	}

	fn parse_tokens(tokens: &mut Peekable<Iter<Token>>) -> Result<Self, ParserError>
			where Self: Sized;
	
	fn keywords() -> &'static [&'static str];
	fn operators() -> &'static [&'static str];
	fn ignore_whitespace() -> bool;
}

#[macro_export]
macro_rules! expect {
	($list: ident, $mtch: pat, $i: ident, $cmp: expr, $msg: literal) => {
		match $list.peek() {
			Some($mtch) if $cmp => {
				$list.next();
				$i
			}
			token => return Err(ParserError::new(format!($msg, token)))
		}
	};
	($list: ident, $mtch: pat, $value: ident, $msg: literal) => {
		match $list.peek() {
			Some($mtch) => {
				$list.next();
				$value
			}
			token => return Err(ParserError::new(format!($msg, token)))
		}
	};
	($list: ident, $mtch: pat, $msg: literal) => {
		match $list.peek() {
			Some($mtch) => {
				$list.next().unwrap()
			}
			token => return Err(ParserError::new(format!($msg, token)))
		}
	};
}

#[macro_export]
macro_rules! expect_break {
	($list: ident, $mtch: pat, $i: ident, $cmp: expr) => {
		match $list.peek() {
			Some($mtch) if $cmp => {
				$list.next();
				$i
			}
			_ => break
		}
	};
	($list: ident, $mtch: pat, $value: ident) => {
		match $list.peek() {
			Some($mtch) => {
				$list.next();
				$value
			}
			_ => break
		}
	};
}