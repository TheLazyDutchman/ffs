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

		Self { column, row, index: value.len() - 1, file, file_id}
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


#[derive(Debug, Clone)]
pub struct Chunk {
	buffer: Peekable<IntoIter<char>>,
	length: usize,
	index: usize,
	row: usize,
	column: usize,
	start_index: usize,
	file: Option<String>,
	file_id: u32
}

impl Chunk {
	pub fn new(buffer: String, row: usize, column: usize, start_index: usize, file: Option<String>, file_id: u32) -> Chunk {
		let length = buffer.len();
		let buffer = buffer.clone().chars().collect::<Vec<_>>();
		let buffer = buffer.into_iter().peekable();
		Chunk { buffer, length, index: 0, row, column, start_index, file, file_id }
	}

	pub fn is_done(&self) -> bool {
		self.index >= self.length
	}

	pub fn position(&self) -> Position {
		Position { column: self.column, row: self.row, index: self.start_index + self.index, file: self.file.clone(), file_id: self.file_id }
	}

	pub fn next(&mut self) -> Option<char> {
		self.index += 1;
		self.column += 1;

		let chr = self.buffer.next();
		match chr {
			Some(chr) if chr == '\n' => {
				self.column = 0;
				self.row += 1;
				Some(chr)
			}
			chr => chr
		}
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.buffer.peek()
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
		let file = self.file.clone();
		let eof = Position::end(&buffer, file.clone(), self.file_id);

		let mut row = 0;
		let mut column = 0;
		let mut start_index = 0;

		let chunks = buffer.split_inclusive(|c: char| c.is_whitespace()).map(|chunk| {
			let mut chunk = chunk.to_owned();
			for c in chunk.chars() {
				column += 1;
				if c == '\n' {
					row += 1;
					column = 0;
				}
			}
			start_index += chunk.len();

			if !self.usewhitespace {
				chunk = chunk.trim().to_string();
			}

			Chunk::new(chunk, row, column, start_index, file.clone(), self.file_id)
		}).collect();
		CharStream { buffer, chunks, chunk_index: 0, file_id: self.file_id, eof }
	}
}

#[derive(Debug, Clone)]
pub struct CharStream {
	buffer: String,
	chunks: Vec<Chunk>,
	chunk_index: usize,
	file_id: u32,
	eof: Position
}

impl CharStream {
	pub fn new(value: String) -> CharStreamBuilder {
		CharStreamBuilder::new(value)
	}

	pub fn get_chunk(&mut self) -> Result<&mut Chunk, ParseError> {
		if self.chunk_index >= self.chunks.len() {
			return Err(ParseError::error("Reached end of file.", self.position()?))
		}
		if self.chunks[self.chunk_index].is_done() {
			self.chunk_index += 1;
			if self.chunk_index >= self.chunks.len() {
				return Err(ParseError::error("Reached end of file.", self.eof.clone()))
			}
		}

		Ok(&mut self.chunks[self.chunk_index])
	}

	pub fn position(&mut self) -> Result<Position, ParseError> {
		Ok(self.get_chunk()?.position())
	}

	pub fn goto(&mut self, position: Position) -> Result<(), ParseError> {
		if self.file_id != position.file_id {
			return Err(ParseError::error("Could not go to position in different buffer.", position));
		}

		if position < self.position()? {
			return Err(ParseError::error("Charstream does not support going back.", position));
		}

		if position > self.eof {
			return Err(ParseError::error("Charstream can not go to position after end of buffer.", self.eof.clone()));
		}

		while self.position()? < position {
			self.get_chunk()?.next();
		}

		Ok(())
	}
}