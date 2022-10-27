use core::fmt;
use std::cmp::Ordering;

use super::ParseError;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: usize,
    collumn: usize,
}

impl Position {
    pub fn new(row: usize, collumn: usize) -> Self {
        Self { row, collumn }
    }

    pub fn advance(&mut self) -> &mut Self {
        self.collumn += 1;
        self
    }

    pub fn newline(&mut self) -> &mut Self {
        self.collumn = 0;
        self.row += 1;
        self
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.collumn)
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.start, self.end)
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let start = self.start.partial_cmp(&other.start);
        let end = self.end.partial_cmp(&other.end);
        match start {
            Some(Ordering::Equal) => end,
            Some(Ordering::Greater) => match end {
                Some(Ordering::Equal) | Some(Ordering::Less) => Some(Ordering::Less),
                _ => None,
            },
            Some(Ordering::Less) => match end {
                Some(Ordering::Equal) | Some(Ordering::Greater) => Some(Ordering::Greater),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum WhitespaceType {
    #[default]
    Ignore,
    KeepAll,
}

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    String,
    Number,
    Token,
    WhiteSpace,
}

pub struct Token {
    pub value: String,
    pub span: Span,
    pub tokentype: TokenType,
}

impl Token {
    pub fn new(value: String, span: Span, tokentype: TokenType) -> Self {
        Self {
            value,
            span,
            tokentype,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({}, from {:?})",
            self.tokentype, self.value, self.span
        )
    }
}

pub trait TokenStream: Iterator<Item = Token> + Clone {
    fn pos(&self) -> Position;
    fn indent(&self) -> usize;
    fn set_whitespace(&mut self, whitespace: WhitespaceType) -> &mut Self;

    fn goto(&mut self, position: Position) -> Result<&mut Self, ParseError> {
        if self.pos() > position {
            return Err(ParseError::new(
                "Can not move backward in a token stream.",
                position,
            ));
        }
        while self.pos() < position {
            self.next();
        }

        Ok(self)
    }
}
