use std::{vec::IntoIter, fs::File};

use super::tokenstream::{TokenStream, Token, Position, TokenType, Span};

pub struct BufferStream {
    position: Position,
    buffer: IntoIter<char>,
    indent: usize,
    do_indent: bool,
    current: Option<char>,
}

impl Iterator for BufferStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(c) if c.is_alphabetic() || c == '_' => self.lex_identifier(),
            Some(c) if c.is_numeric() => self.lex_number(),
            Some(c) if c.is_whitespace() => self.lex_whitespace(),
            Some(c) if c == char::default() => None,
            _ => todo!()
        }
    }
}

impl TokenStream for BufferStream {
    fn pos(&self) -> Position {
        self.position.clone()
    }
}

impl BufferStream {
    pub fn new(buffer: String) -> Self {
        let mut buffer = buffer.chars().collect::<Vec<_>>().into_iter();
        Self {
            position: Position::default(),
            buffer, 
            indent: 0,
            do_indent: false,
            current: buffer.next()
        }
    }

    fn lex_identifier(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();

        loop {
            match self.current {
                Some(c) if c.is_alphanumeric() || c == '_' => {
                    value.push(c);
                    self.current = self.buffer.next();
                }
                _ => break
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::Identifier))
    }
    
    fn lex_number(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();

        loop {
            match self.current {
                Some(c) if c.is_numeric() => {
                    value.push(c);
                    self.current = self.buffer.next();
                }
                _ => break
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::Number))
    }
    
    fn lex_whitespace(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();

        loop {
            match self.current {
                Some(c) if c.is_whitespace() => {
                    value.push(c);
                    self.current = self.buffer.next();
                }
                _ => break
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::WhiteSpace))
    }
}

impl<T: AsRef<str>> From<T> for BufferStream {
    fn from(value: T) -> Self {
        Self::new(value.as_ref().to_owned())
    }
}
