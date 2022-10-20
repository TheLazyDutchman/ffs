#![allow(unused)]

use std::{fs, collections::HashMap};

use parseal::{parsing::{self, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Number, StringValue, Parse, charstream::CharStream, Identifier}, Parsable, data_formats::{TreeData, ParseNode, NamedValue, Node}};

#[derive(Clone, Parsable, Debug)]
pub struct JSON {
	value: ParseNode<<JSON as TreeData>::Object, <JSON as TreeData>::List>
}

impl TreeData for JSON {
    type Object = Group<Brace, List<NamedValue<StringValue, Colon, JSON>, Comma>>;

    type List = Group<Bracket, List<JSON, Comma>>;

    fn value(&self) -> ParseNode<Self::Object, Self::List> {
        self.value.clone()
    }
}

fn main() {
	let file = fs::read_to_string("examples/json/example.json")
		.expect("Expected example file to exist.");

	let mut buffer = CharStream::new(file).build();
	let value = JSON::parse(&mut buffer).unwrap();

	let node: Node = value.clone().into();
	println!("value: {:#?}", value);
	println!("node: {:#?}", node);
}