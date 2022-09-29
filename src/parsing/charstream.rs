use std::{str::Chars, iter::Peekable};

use super::ParseError;

#[derive(Debug)]
pub struct Position {
	column: usize,
	row: usize,
	index: usize,
	buffer: String
}

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
	column: usize,
	row: usize,
	index: usize,
	buffer: String,
	chars: Peekable<Chars<'a>>
}

impl<'a> CharStream<'a> {
	pub fn new(value: &'a str) -> Self {
		let buffer = value.to_owned();
		let chars = value.chars().peekable();
		Self { column: 0, row: 0, index: 0, buffer, chars }
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.chars.peek()
	}

	pub fn next(&mut self) -> Option<char> {
		self.index += 1;
		self.column += 1;

		let char = self.chars.next();
		match char {
			Some(char) if char == '\n' => {
				self.column = 0;
				self.row += 1;
				
				Some(char)
			}
			char => char
		}
	}

	pub fn position(&self) -> Position {
		Position { column: self.column, row: self.row, index: self.index, buffer: self.buffer.clone() }
	}

	pub fn goto(&mut self, position: Position) -> Result<(), ParseError> {
		if self.buffer != position.buffer {
			return Err(ParseError::error("Could not go to position in different buffer.", position));
		}

		if position.index < self.index {
			return Err(ParseError::error("Charstream does not support going back.", position));
		}

		while self.index < position.index {
			self.next();
		}

		Ok(())
	}
}

impl<'a> From<&'a str> for CharStream<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
