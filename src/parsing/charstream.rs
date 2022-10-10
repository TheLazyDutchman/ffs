use std::{vec::IntoIter, fmt};
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

		Self { column, row, index: value.len(), file, file_id }
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

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.file {
			Some(file) => write!(f,"{}:{}:{}", file, self.row, self.column),
			None => write!(f, "{}:{}", self.row, self.column)
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
	pub start: Position,
	pub end: Position
}

impl Span {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	pub fn empty(value: &CharStream) -> Self {
		Self { start: value.position(), end: value.position() }
	}
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.end.partial_cmp(&other.end)
    }
}

impl fmt::Display for Span {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} - {}:{}", self.start, self.start.row, self.start.column)
	}
}

#[derive(Debug, Clone)]
pub enum WhitespaceType {
	Ignore,
	KeepAll,
	Indent
}

pub struct CharStreamBuilder {
	buffer: String,
	file: Option<String>,
	file_id: u32
}

impl CharStreamBuilder {
	pub fn new(buffer: String) -> Self {
		Self { buffer, file: None, file_id: random() }
	}

	pub fn build(&mut self) -> CharStream {
		let buffer = self.buffer.clone();
		let chars = buffer.chars().collect::<Vec<_>>().into_iter();
		let file = self.file.clone();
		let eof = Position::end(&buffer, file.clone(), self.file_id);


		CharStream { chars, file, file_id: self.file_id, column: 0, row: 0, index: 0, eof, whitespace: WhitespaceType::Ignore }
	}
}

#[derive(Debug, Clone)]
pub struct CharStream {
	chars: IntoIter<char>,
	file: Option<String>,
	file_id: u32,
	column: usize,
	row: usize,
	index: usize,
	eof: Position,
	whitespace: WhitespaceType
}

impl CharStream {
	pub fn new(value: String) -> CharStreamBuilder {
		CharStreamBuilder::new(value)
	}

	pub fn position(&self) -> Position {
		Position { column: self.column, row: self.row, index: self.index, file: self.file.clone(), file_id: self.file_id }
	}

	pub fn next(&mut self) -> Option<char> {
		let chr = match self.chars.next() {
			Some('\n') => {
				self.index += 1;
				self.column = 0;
				self.row += 1;
				Some('\n')
			}
			Some(value) => {
				self.index += 1;
				self.column += 1;
				Some(value)
			}
			None => None
		};

		match self.whitespace {
			WhitespaceType::Ignore => {
				match chr {
					Some(c) if c.is_whitespace() => {
						self.next()
					}
					c => c,
				}
			}
			WhitespaceType::KeepAll => chr,
			WhitespaceType::Indent => {
				todo!()
			}
		}
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

	pub fn set_whitespace(&mut self, whitespace: WhitespaceType) {
		self.whitespace = whitespace;
	}
}