use pass_macro_attribute::{parsable, pass};
use crate::parsing::Parsable;
use std::slice::Iter;

#[parsable()]
#[pass(StartLabel, "{operator,<} {identifier} {operator,>}")]
#[pass(EndLabel, "{operator,</} {identifier} {operator,>}")]
#[pass(Scope, "{startlabel} {endlabel}")]
pub struct HTML {

}

impl HTML {
	pub fn parse(value: String) -> Self {
		let tokens = Self::parse_tokens(value);
		let mut scopes = Vec::new();

		while tokens.len() > 0 {
			scopes.push(Self::parse_scope(tokens.clone().map(|t| t.clone()).collect()));
		}

		Self { tokens: scopes }
	}
}