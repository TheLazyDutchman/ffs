use std::vec::IntoIter;

use super::tokenstream::{Position, Span, Token, TokenStream, TokenType, WhitespaceType};

#[derive(Clone)]
pub struct BufferStream {
    position: Position,
    buffer: IntoIter<char>,
    indent: usize,
    do_indent: bool,
    current: Option<char>,
    whitespace: WhitespaceType,
}

impl Iterator for BufferStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(c) if c.is_alphabetic() || c == '_' => self.lex_identifier(),
            Some('"') | Some('\'') => self.lex_string(),
            Some(c) if c.is_numeric() => self.lex_number(),
            Some(c) if c.is_whitespace() => {
                let whitespace = self.lex_whitespace();
                if self.whitespace == WhitespaceType::Ignore {
                    self.next()
                } else {
                    whitespace
                }
            },
            Some(_) => self.lex_token(),
            None => None,
        }
    }
}

impl TokenStream for BufferStream {
    fn pos(&self) -> Position {
        self.position.clone()
    }

    fn indent(&self) -> usize {
        self.indent
    }

    fn set_whitespace(&mut self, whitespace: WhitespaceType) -> &mut Self {
        self.whitespace = whitespace;
        self
    }
}

impl BufferStream {
    pub fn new(buffer: String) -> Self {
        let mut buffer = buffer.chars().collect::<Vec<_>>().into_iter();
        let current = buffer.next();
        Self {
            position: Position::default(),
            buffer,
            indent: 0,
            do_indent: false,
            current,
            whitespace: WhitespaceType::default(),
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
                    self.position.advance();
                }
                _ => break,
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::Identifier))
    }

    fn lex_string(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();
        let delim = self.current.unwrap();

        value.push(self.current.unwrap());
        self.current = self.buffer.next();
        self.position.advance();

        loop {
            match self.current {
                Some(c) if c == delim => {
                    value.push(c);
                    self.current = self.buffer.next();
                    self.position.advance();
                    break;
                }
                Some(c) => {
                    value.push(c);
                    self.current = self.buffer.next();
                    self.position.advance();
                }
                None => return None,
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::String))
    }

    fn lex_number(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();

        loop {
            match self.current {
                Some(c) if c.is_numeric() => {
                    value.push(c);
                    self.current = self.buffer.next();
                    self.position.advance();
                }
                _ => break,
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::Number))
    }

    fn lex_whitespace(&mut self) -> Option<Token> {
        let mut value = String::new();
        let start = self.pos();

        loop {
            if self.do_indent {
                match self.current {
                    Some(' ') => self.indent += 1,
                    Some('\t') => self.indent += 4,
                    _ => {}
                }
            }

            match self.current {
                Some('\n') => {
                    self.indent = 0;
                    value.push('\n');
                    self.current = self.buffer.next();
                    self.position.newline();
                }
                Some(c) if c.is_whitespace() => {
                    value.push(c);
                    self.current = self.buffer.next();
                    self.position.advance();
                }
                _ => {
                    self.do_indent = false;
                    break;
                }
            }
        }

        let span = Span::new(start, self.pos());
        Some(Token::new(value, span, TokenType::WhiteSpace))
    }

    fn lex_token(&mut self) -> Option<Token> {
        let value = String::from(self.current.unwrap());
        let start = self.pos();
        self.current = self.buffer.next();
        self.position.advance();

        let span = Span::new(start, self.pos());
        let token = Token::new(value, span, TokenType::Token);
        Some(token)
    }
}

impl<T: AsRef<str>> From<T> for BufferStream {
    fn from(value: T) -> Self {
        Self::new(value.as_ref().to_owned())
    }
}
