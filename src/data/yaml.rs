use std::collections::HashMap;

use crate::parsing::{AST, token::{Token, TokenStream}, ParserError};

use super::{Data, TreeData};

#[derive(Debug, PartialEq)]
pub struct YAML {
    pub value: Data
}

impl AST for YAML {
    fn parse(filename: String) -> Result<Self, crate::parsing::ParserError> 
            where 
            Self: Sized {
        let mut tokens = TokenStream::parse(filename)?;
        tokens.remove_whitespace();

        let mut tokens = tokens.iter().peekable();

        for _ in 0..=2 {
            match tokens.peek() {
                Some(Token::Char(ch)) if *ch == '-' => {
                    tokens.next();
                }
                _ => {
                    return Err(ParserError::new("Expected --- at the beginning of a yaml file".to_owned()));
                }
            }
        }

        Ok(Self { value: Self::parse_data(&mut tokens)? })
    }
}

impl TreeData for YAML {
    fn parse_data(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Data, crate::parsing::ParserError> {
        match tokens.peek().ok_or(ParserError::eof())? {
            Token::Char(ch) if *ch == '-' => {
                Self::parse_list(tokens)
            }
            Token::Identifier(_) => {
                Self::parse_object(tokens)
            }
            Token::String(_) | Token::Number(_) => {
                Ok(Data::Immediate(tokens.next().unwrap().clone()))
            }
            token => {
                Err(ParserError::new(format!("Unexpected token '{:?}'", token).to_owned()))
            }
        }
    }

    fn parse_list(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Data, crate::parsing::ParserError> {
        let mut values = Vec::new();

        loop {
            match tokens.peek() {
                Some(Token::Char(ch)) if *ch == '-' => {
                    tokens.next();
                }
                _ => break
            }

            values.push(Self::parse_data(tokens)?);
        }

        Ok( Data::List(values) )
    }

    fn parse_object(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Data, crate::parsing::ParserError> {
        let mut map = HashMap::new();

        loop {
            let name = match tokens.peek() {
                Some(Token::Identifier(value)) => {
                    tokens.next();
                    value.to_owned()
                }
                _ => break
            };

            match tokens.peek() {
                Some(Token::Char(ch)) if *ch == ':' => {
                    tokens.next();
                }
                token => return Err(ParserError::new(format!("Expected ':' but got '{:?}'", token)))
            }

            let value = Self::parse_data(tokens)?;

            map.insert(name, value);
        }

        Ok(Data::Object(map))
    }
}