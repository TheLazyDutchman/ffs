use std::{iter::Peekable, slice::Iter};

use crate::parsing::{ParserError, token::Token};

pub mod html;

pub trait MarkupToken: From<Token> + Clone {
}

#[derive(Debug, PartialEq, Clone)]
pub struct MarkupTokenStream<T> where T: MarkupToken {
	values: Vec<T>
}

impl<T> MarkupTokenStream<T> where T: MarkupToken {
	fn iter(&self) -> Peekable<Iter<T>> {
		self.values.iter().peekable()
	}
}

impl<'a, T> From<&mut Peekable<Iter<'a, Token>>> for MarkupTokenStream<T> where T: MarkupToken {
    fn from(tokens: &mut Peekable<Iter<Token>>) -> Self {
        let mut values = Vec::new();

		while tokens.len() > 0 {
			values.push(tokens.next().unwrap().to_owned().into());
		}

		Self {values}
    }
}

impl<T> From<Vec<T>> for MarkupTokenStream<T> where T: MarkupToken {
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

pub trait Markup {
	type Token: MarkupToken;

	fn parse_labels(tokens: MarkupTokenStream<Self::Token>) 
		-> Result<MarkupTokenStream<Self::Token>, ParserError>;
	fn parse_scopes(tokens: MarkupTokenStream<Self::Token>)
		-> Result<MarkupTokenStream<Self::Token>, ParserError>;
}