use pass_macro_attribute::{parsable, pass};
use crate::parsing::{Parsable, ParserError};


#[parsable()]
#[pass(StartLabel, "start:{Operator,<} ident:{Identifier} end:{Operator,>}")]
#[pass(EndLabel, "start:{Operator,</} ident:{Identifier} end:{Operator,>}")]
#[pass(Scope, "start:{StartLabel} end:{EndLabel}")]
pub struct HTML {

}

impl HTML {
	pub fn parse(value: String) -> Result<Self, ParserError> {
		let tokens = Self::parse_tokens(value);
		let tokens = tokens.iter();
		let mut scopes = Vec::new();

		while tokens.len() > 0 {
			scopes.push(Token::Scope(Self::parse_scope(tokens.clone().map(|t| t.clone()).collect())?));
		}

		Ok(Self { tokens: scopes })
	}
}