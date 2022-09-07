use std::{collections::HashMap, slice::Iter, iter::Peekable};

use crate::parsing::{AST, token::{Token}, ParserError};

use super::{Data, TreeData};

#[derive(Debug, PartialEq)]
pub struct YAML {
	pub value: Data
}

impl AST for YAML {
	fn parse_tokens(tokens: &mut Peekable<Iter<Token>>) -> Result<Self, ParserError>
			where Self: Sized {
		
		match tokens.peek() {
			Some(Token::Operator(op)) if op == "---" => {
				tokens.next();
			}
			token => {
				return Err(ParserError::new(format!("Expected '---' at the start of a yaml file, got '{:?}'", token)))
			}
		}
        Ok(Self{value: Self::parse_data(tokens)?})
    }

	fn keywords() -> &'static [&'static str] {
        &["true", "false"]
    }

	fn operators() -> &'static [&'static str] {
	   &["---", "-", ":"]
    }

	fn ignore_whitespace() -> bool {
		  true
    }
}

impl TreeData for YAML {
	fn parse_data(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError> {
		match tokens.peek().ok_or(ParserError::eof())? {
			Token::Operator(op) if *op == "-" => {
				Self::parse_list(tokens)
			}
			Token::Identifier(_) => {
				Self::parse_object(tokens)
			}
			Token::String(_) | Token::Number(_) | Token::Keyword(_) => {
				Ok(Data::Immediate(tokens.next().unwrap().clone()))
			}
			token => {
				Err(ParserError::new(format!("Unexpected token '{:?}'", token).to_owned()))
			}
		}
	}

	fn parse_list(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError> {
		let mut values = Vec::new();

		loop {
			match tokens.peek() {
				Some(Token::Operator(op)) if *op == "-" => {
					tokens.next();
				}
				_ => break
			}

			values.push(Self::parse_data(tokens)?);
		}

		Ok( Data::List(values) )
	}

	fn parse_object(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError> {
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
				Some(Token::Operator(op)) if *op == ":" => {
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