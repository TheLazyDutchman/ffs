#![allow(unused)]

use std::{fs, collections::HashMap};

use parseal::{parsing::{self, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Number, StringValue, Parse, charstream::CharStream, Identifier}, Parsable};
use parseal::data_formats::{ParseNode, Node, NamedValue};

#[derive(Clone, Parsable, Debug)]
pub struct JSON {
	value: ParseNode<
			Group<Brace, List<NamedValue<Colon, JSON>, Comma>>,
			Group<Bracket, List<JSON, Comma>>
		>
}

impl From<JSON> for ParseNode<
			Group<Brace, List<NamedValue<Colon, JSON>, Comma>>,
			Group<Bracket, List<JSON, Comma>>
		>
{
	fn from(json: JSON) -> Self {
	    json.value
	}
}

fn main() {
	let file = fs::read_to_string("examples/json/example.json")
		.expect("Expected example file to exist.");

	let mut buffer = CharStream::new(file).build();
	let value = JSON::parse(&mut buffer).unwrap();

	let node: Node = <ParseNode<
			Group<Brace, List<NamedValue<Colon, JSON>, Comma>>,
			Group<Bracket, List<JSON, Comma>>
		> as Into<Node>>::into(value.clone().into());
	println!("value: {:#?}", value);
	println!("node: {:#?}", node);
}
