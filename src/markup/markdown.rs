#![allow(unused)]

use pass_macro_attribute::{pass, parsable};
use crate::parsing::Parsable;
use std::slice::Iter;


#[parsable()]
#[pass(Bold, "* {text} *")]
#[pass(Italic, "- {text} -")]
pub struct MarkDown {
}

impl MarkDown {
	pub fn new(tokens: Vec<<Self as Parsable>::Token>) -> Self {
		todo!()
		// Self { tokens: Self::parse_bold(tokens) }
	}

	pub fn parse_text(tokens: Vec<<Self as Parsable>::Token>) -> Token {
		todo!()
	}
}