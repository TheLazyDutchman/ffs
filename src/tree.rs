use std::iter::Peekable;

use crate::{parsing::ParserError, pass::Pass};

#[derive(Debug, Clone)]
pub enum TreeToken<T> where T: Clone {
	Token(T),
	Scope(TreeTokenStream<T>)
}

#[derive(Debug, Clone)]
pub struct TreeTokenStream<T> where T: Clone {
	tokens: Vec<TreeToken<T>>,
	index: usize
}

impl<T> TreeTokenStream<T> where T: Clone {
	fn new(tokens: Vec<TreeToken<T>>) -> Self {
		Self { tokens, index: 0 }
	}
}

impl<T> Iterator for TreeTokenStream<T> where T: Clone {
    type Item = TreeToken<T>;

    fn next(&mut self) -> Option<Self::Item> {
		if self.tokens.len() == self.index {
			return None;
		}

		self.index += 1;
		Some(self.tokens[self.index - 1].clone())
    }
}

impl<T> From<Vec<TreeToken<T>>> for TreeTokenStream<T> where T: Clone {
    fn from(tokens: Vec<TreeToken<T>>) -> Self {
        Self::new(tokens)
    }
}

pub trait Tree {
	type Token: Clone;
	fn is_scope_start(tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool where Self: Sized;
	fn is_scope_end(tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool where Self: Sized;
	
	fn new (tokens: Vec<Self::Token>) -> Result<Self, ParserError> where Self: Sized {
		let tokens: Vec<TreeToken<Self::Token>> = tokens.iter().map(|t| TreeToken::Token(t.clone())).collect();
		let tokens: TreeTokenStream<Self::Token> = tokens.into();

		Self::parse(tokens.peekable())
	}

	fn parse(tokens: Peekable<TreeTokenStream<Self::Token>>) -> Result<Self, ParserError> where Self: Sized;

	fn parse_scope(tokens: &mut Peekable<TreeTokenStream<Self::Token>>) -> Result<TreeToken<Self::Token>, ParserError>  where Self: Sized {
		if Self::is_scope_start(tokens.clone()) {
			let mut inner = Vec::new();

			inner.push(tokens.next().unwrap());

			while !Self::is_scope_end(tokens.clone()) {
				inner.push(Self::parse_scope(tokens)?);
			}

			inner.push(tokens.next().ok_or(ParserError::new("Expected scope end.".to_owned()))?);
			
			return Ok(TreeToken::Scope(inner.into()));
		}

		Ok(tokens.next().unwrap())
	}

}

impl<T> Pass for dyn Tree<Token = T> {

}