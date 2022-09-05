use std::{slice::Iter, iter::Peekable};

use super::{AST, Chars, ParserError};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>
}

impl TokenStream {
    pub fn iter(&self) -> Iter<Token> {
        self.tokens.iter()
    }

    pub fn remove_whitespace(&mut self) -> &Self {
        let mut tokens = Vec::new();

        for token in self.tokens.iter() {
            match token {
                Token::WhiteSpace(_) => {}
                _ => {
                    tokens.push(token.clone());
                }
            }
        }

        self.tokens = tokens;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    value: usize,
    base: u8
}

impl Number {
    pub fn new(value: usize, base: u8) -> Self {
        Self { value, base }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    WhiteSpace(String),
    Identifier(String),
    String(String),
    Number(Number),
    Char(char)
}

impl Token {
    fn lex_whitespace(chars: &mut Peekable<Iter<char>>) -> Token {
        let mut value = String::new();

        while chars.len() > 0 && chars.peek().unwrap().is_whitespace() {
            value.push(*chars.next().unwrap());
        }

        Token::WhiteSpace(value)
    }

    fn lex_identifier(chars: &mut Peekable<Iter<char>>) -> Token {
        let mut value = String::new();

        while chars.len() > 0 &&
            (chars.peek().unwrap().is_alphanumeric() ||
            chars.peek().unwrap() == &&'_') {
                value.push(*chars.next().unwrap());
        }

        Token::Identifier(value)
    }

    fn lex_number(chars: &mut Peekable<Iter<char>>) -> Token {
        let mut string = String::new();

        while chars.len() > 0 && chars.peek().unwrap().is_numeric() {
            string.push(*chars.next().unwrap());
        }

        if chars.peek().unwrap() == &&'x' || chars.peek().unwrap() == &&'b' {
            todo!()
        }

        let value = string.parse().unwrap();
        let base = 10;

        Token::Number(Number{value, base})
    }

    fn lex_string(chars: &mut Peekable<Iter<char>>) -> Token {
        let ch = chars.next().unwrap();

        let mut value = String::from(*ch);

        while value.len() > 0 {
            if chars.peek().unwrap() == &&'\\' {
                value.push(*chars.next().unwrap());
            } else if chars.peek().unwrap() == &ch {
                break;
            }

            value.push(*chars.next().unwrap());
        }

        value.push(*chars.next().unwrap());

        Self::String(value)
    }
}

impl AST for TokenStream {
    fn parse(filename: String) -> Result<Self, ParserError> 
            where 
            Self: Sized {
        let chars = Chars::parse(filename)?;
        let mut chars = chars.chars.iter().peekable();
        let mut tokens = Vec::new();

        while chars.len() > 0 {
            match chars.peek() {
                Some(ch) if ch.is_whitespace() => {
                    tokens.push(Token::lex_whitespace(&mut chars));
                }
                Some(&&ch) if ch.is_alphabetic() || ch == '_' => {
                    tokens.push(Token::lex_identifier(&mut chars));
                }
                Some(&&ch) if ch == '"' || ch == '\'' => {
                    tokens.push(Token::lex_string(&mut chars));
                }
                Some(ch) if ch.is_numeric() => {
                    tokens.push(Token::lex_number(&mut chars));
                }
                Some(_) => {
                    tokens.push(Token::Char(*chars.next().unwrap()));
                }
                None => {}
            }
        }

        Ok(Self { tokens })
    }
}