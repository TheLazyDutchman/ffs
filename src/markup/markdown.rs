#![allow(unused)]

use pass_macro_attribute::{pass, parsable};
use crate::parsing::{Parsable, ParserError};
use std::slice::Iter;

#[derive(Clone)]
pub struct Text;

#[parsable()]
#[pass(Bold, "start:* value:{Text} end:*")]
#[pass(Italic, "start:- value:{Text} end:-")]
pub struct MarkDown {
}

impl MarkDown {
	pub fn new(tokens: Vec<<Self as Parsable>::Token>) -> Self {
		todo!()
		// Self { tokens: Self::parse_bold(tokens) }
	}

	pub fn parse_text(tokens: Vec<<Self as Parsable>::Token>) -> Result<Text, ParserError> {
		todo!()
	}
}