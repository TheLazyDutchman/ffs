use std::{iter::Peekable, vec::IntoIter};
use rand::random;

use super::ParseError;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
	pub column: usize,
	pub	row: usize,
	pub	index: usize,
	pub	file: Option<String>,
	pub file_id: u32
}

impl Position {
	pub fn end(value: &str, file: Option<String>, file_id: u32) -> Position {
		let mut column = 0;
		let mut row = 0;

		for c in value.chars() {
			match c {
				'\n' => {
					column = 0;
					row += 1;
				}
				_ => column += 1,
			}
		}

		Self { column, row, index: value.len() - 1, file, file_id }
	}
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self.file.partial_cmp(&other.file) {
			Some(core::cmp::Ordering::Equal) => {}
			_ => return None,
		}
		match self.file_id.partial_cmp(&other.file_id) {
			Some(core::cmp::Ordering::Equal) => {}
			_ => return None,
		}

        match self.row.partial_cmp(&other.row) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}


pub struct CharStreamBuilder {
	buffer: String,
	usewhitespace: bool,
	file: Option<String>,
	file_id: u32
}

impl CharStreamBuilder {
	pub fn new(buffer: String) -> Self {
		Self { buffer, usewhitespace: false, file: None, file_id: random() }
	}

	pub fn build(&mut self) -> CharStream {
		let buffer = self.buffer.clone();
		let chars = buffer.chars().collect::<Vec<_>>().into_iter().peekable();
		let file = self.file.clone();
		let eof = Position::end(&buffer, file.clone(), self.file_id);


		CharStream { buffer, chars, file, file_id: self.file_id, previous: None, column: 0, row: 0, index: 0, eof }
	}
}

#[derive(Debug, Clone)]
pub struct CharStream {
	buffer: String,
	chars: Peekable<IntoIter<char>>,
	file: Option<String>,
	file_id: u32,
	previous: Option<char>,
	column: usize,
	row: usize,
	index: usize,
	eof: Position
}

impl CharStream {
	pub fn new(value: String) -> CharStreamBuilder {
		CharStreamBuilder::new(value)
	}

	pub fn position(&self) -> Position {
		Position { column: self.column, row: self.row, index: self.index, file: self.file.clone(), file_id: self.file_id }
	}

	pub fn next(&mut self) -> Option<char> {
		match self.previous {
			Some('\n') => {
				self.index += 1;
				self.column = 0;
				self.row += 1;
			}
			Some(c) => {
				self.index += 1;
				self.column += 1;
			}
			None => {}
		}

		self.previous = match self.chars.next() {
			Some('\n') => {
				self.row += 1;
				self.column = 0;
				Some('\n')
			}
			c => c
		};

		self.previous
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.chars.peek()
	}

	pub fn goto(&mut self, position: Position) -> Result<(), ParseError> {
		if self.file_id != position.file_id {
			return Err(ParseError::error("Could not go to position in different buffer.", position));
		}

		if position < self.position() {
			return Err(ParseError::error("Charstream does not support going back.", position));
		}

		if position > self.eof {
			return Err(ParseError::error("Charstream can not go to position after end of buffer.", self.eof.clone()));
		}

		while self.position() < position {
			self.next();
		}

		Ok(())
	}
}