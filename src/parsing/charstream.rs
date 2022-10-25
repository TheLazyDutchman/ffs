use rand::random;
use std::{fmt, vec::IntoIter};

use super::ParseError;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Position {
    pub column: usize,
    pub row: usize,
    pub index: usize,
    pub file: Option<String>,
    pub file_id: u32,
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

        Self {
            column,
            row,
            index: value.len(),
            file,
            file_id,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other == &Position::default() {
            return Some(std::cmp::Ordering::Greater);
        }
        if self == &Position::default() {
            return Some(std::cmp::Ordering::Less);
        }

        match self.file.partial_cmp(&other.file) {
            Some(std::cmp::Ordering::Equal) => {}
            _ => return None,
        }
        match self.file_id.partial_cmp(&other.file_id) {
            Some(std::cmp::Ordering::Equal) => {}
            _ => return None,
        }

        match self.row.partial_cmp(&other.row) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{}:{}:{}", file, self.row, self.column),
            None => write!(f, "{}:{}", self.row, self.column),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other == &Span::default() {
            return Some(std::cmp::Ordering::Greater)
        }
        if self == &Span::default() {
            return Some(std::cmp::Ordering::Less)
        }

        match self.start.partial_cmp(&other.start) {
            Some(std::cmp::Ordering::Less) => match self.end.partial_cmp(&other.end) {
                Some(std::cmp::Ordering::Less) | None => None,
                Some(std::cmp::Ordering::Equal) | Some(std::cmp::Ordering::Greater) => {
                    Some(std::cmp::Ordering::Greater)
                }
            },
            Some(std::cmp::Ordering::Equal) => self.end.partial_cmp(&other.end),
            Some(std::cmp::Ordering::Greater) => match self.end.partial_cmp(&other.end) {
                Some(std::cmp::Ordering::Greater) | None => None,
                Some(std::cmp::Ordering::Equal) | Some(std::cmp::Ordering::Less) => {
                    Some(std::cmp::Ordering::Less)
                }
            },
            None => None,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}:{}", self.start, self.end.row, self.end.column)
    }
}

#[derive(Debug, Clone)]
pub enum WhitespaceType {
    Ignore,
    KeepAll,
    Indent,
}

pub struct CharStreamBuilder {
    buffer: String,
    file: Option<String>,
    file_id: u32,
    indent_size: u8,
}

impl CharStreamBuilder {
    pub fn new(buffer: String) -> Self {
        Self {
            buffer,
            file: None,
            file_id: random(),
            indent_size: 4,
        }
    }

    pub fn build(&mut self) -> CharStream {
        let buffer = self.buffer.clone();
        let chars = buffer.chars().collect::<Vec<_>>().into_iter();
        let file = self.file.clone();
        let eof = Position::end(&buffer, file.clone(), self.file_id);

        CharStream {
            chars,
            file,
            file_id: self.file_id,
            column: 0,
            row: 0,
            index: 0,
            eof,
            whitespace: WhitespaceType::Ignore,
            indent: 0,
            indent_size: self.indent_size,
            in_indent: true,
        }
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
    whitespace: WhitespaceType,
    indent: u8,
    indent_size: u8,
    in_indent: bool,
}

impl CharStream {
    pub fn new(value: String) -> CharStreamBuilder {
        CharStreamBuilder::new(value)
    }

    pub fn pos(&self) -> Position {
        Position {
            column: self.column,
            row: self.row,
            index: self.index,
            file: self.file.clone(),
            file_id: self.file_id,
        }
    }

    pub fn goto(&mut self, position: Position) -> Result<(), ParseError> {
        if self.file_id != position.file_id {
            return Err(ParseError(
                "Could not go to position in different buffer.".to_string(),
                position,
            ));
        }

        if position < self.pos() {
            return Err(ParseError(
                "Charstream does not support going back.".to_string(),
                position,
            ));
        }

        if position > self.eof {
            return Err(ParseError(
                "Charstream can not go to position after end of buffer.".to_string(),
                self.eof.clone(),
            ));
        }

        while self.pos() < position {
            self.next();
        }

        Ok(())
    }

    pub fn set_whitespace(&mut self, whitespace: WhitespaceType) {
        self.whitespace = whitespace;
    }

    pub fn indent(&self) -> u8 {
        self.indent
    }
}

impl Iterator for CharStream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
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
            None => None,
        };

        match self.whitespace {
            WhitespaceType::Ignore => match chr {
                Some(c) if c.is_whitespace() => self.next(),
                c => c,
            },
            WhitespaceType::KeepAll => chr,
            WhitespaceType::Indent => match chr {
                Some(c) if c.is_whitespace() => {
                    match c {
                        '\t' => {
                            if self.in_indent {
                                self.indent += self.indent_size;
                            }
                        }
                        ' ' => {
                            if self.in_indent {
                                self.indent += 1;
                            }
                        }
                        '\n' => {
                            self.in_indent = true;
                            self.indent = 0;
                        }
                        _ => {}
                    }
                    self.next()
                }
                c => {
                    self.in_indent = false;
                    c
                }
            },
        }
    }
}
