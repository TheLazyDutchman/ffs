use std::iter::Peekable;

use crate::{tree::{Tree, TreeTokenStream, TreeToken}, parsing::{ParserError, token::Token}};

#[derive(Debug, Clone)]
pub struct Label {
	tokens: Vec<TreeToken<Token>>,
	is_start: bool
}

impl Label {
	fn new(tokens: TreeTokenStream<Token>) -> Self {
		let tokens: Vec<TreeToken<Token>> = tokens.collect();

		let is_start = match tokens[0].clone() {
			TreeToken::Token(Token::Operator(op)) if op == "<" => true,
			_ => false
		};

		Self { tokens, is_start }
	}
}

#[derive(Debug, Clone)]
pub enum HTMLToken {
	Token(Token),
	Label(Label)
}

pub struct HTMLLabels(Vec<TreeToken<HTMLToken>>);

impl Tree for HTMLLabels {
    type Token = Token;

    fn is_scope_start(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool {
        match tokens.peek() {
			Some(TreeToken::Token(Token::Operator(op))) if op == "<" || op == "</" => true,
			_ => false
		}
    }

    fn is_scope_end(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool {
        match tokens.peek() {
			Some(TreeToken::Token(Token::Operator(op))) if op == ">" => true,
			_ => false
		}
    }

    fn parse(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> Result<Self, ParserError> where Self: Sized {
        let mut html_tokens = Vec::new();

		while tokens.peek().is_some() {
			html_tokens.push(Self::parse_scope(&mut tokens)?);
		}

		let html_tokens = html_tokens.iter().map(|t| match t {
			TreeToken::Token(value) => TreeToken::Token(HTMLToken::Token(value.clone())),
			TreeToken::Scope(value) => TreeToken::Token(HTMLToken::Label(Label::new(value.clone())))
		}).collect();

		Ok(Self(html_tokens))
    }
}

pub struct HTML {
	tokens: Vec<TreeToken<HTMLToken>>
}

impl HTML {
	fn new(tokens: Vec<Token>) -> Result<Self, ParserError> {
		let HTMLLabels(tokens) = HTMLLabels::new(tokens)?;
		Ok(Self { tokens })
	}
}

impl Tree for HTML {
    type Token = HTMLToken;

    fn is_scope_start(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool {
        match tokens.peek() {
			Some(TreeToken::Token(HTMLToken::Label(label))) if label.is_start => true,
			_ => false
		}
    }

    fn is_scope_end(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> bool {
        match tokens.peek() {
			Some(TreeToken::Token(HTMLToken::Label(label))) if !label.is_start => true,
			_ => false
		}
    }

    fn parse(mut tokens: Peekable<TreeTokenStream<Self::Token>>) -> Result<Self, ParserError> where Self: Sized {
		let mut html_tokens = Vec::new();

		while tokens.peek().is_some() {
			html_tokens.push(Self::parse_scope(&mut tokens)?);
		}

        Ok(Self { tokens: html_tokens })
    }
}