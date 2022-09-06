use std::{error::Error, fmt::Display};

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

pub trait AST {
	fn parse(filename: String) -> Result<Self, ParserError> 
			where 
			Self: Sized;
}

pub trait Pass where Self::In: AST, Self::Out: AST {
	type In;
	type Out;
}

pub struct Chars {
	chars: Vec<char>
}

impl AST for Chars {
	fn parse(filename: String) -> Result<Self, ParserError> 
			where 
			Self: Sized {
		Ok(Self { chars: filename.chars().collect() })
	}
}