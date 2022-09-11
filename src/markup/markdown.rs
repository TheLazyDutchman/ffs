use pass_macro_attribute::pass;
use crate::parsing::Parsable;


#[pass(bold, "*{text}*")]
#[pass(italic, "-{text}-")]
pub struct MarkDown {
}

impl MarkDown {
	pub fn new(tokens: Vec<<Self as Parsable>::Token>) -> Self {
		Self { tokens: Self::parse_bold(tokens) }
	}
}

impl Parsable for MarkDown {
    type Token = String;
}