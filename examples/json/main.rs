#![allow(unused)]

use std::fs;

use parseal::{parsing::{self, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Number, StringValue, Parse, charstream::CharStream, Identifier}, Parsable};

#[derive(Debug, Parsable, Clone)]
pub struct JSONList {
	list: Group<Bracket,
		List<JSONNode, Comma>>
}

#[derive(Debug, Parsable, Clone)]
pub struct NamedValue {
	name: StringValue,
	colon: Colon,
	value: JSONNode
}

#[derive(Debug, Parsable, Clone)]
pub struct JSONObject {
	map: Group<Brace,
		List<NamedValue, Comma>>
}

#[derive(Debug, Parsable, Clone)]
pub enum Value {
	String(StringValue),
	Number(Number),
	Bool(Identifier)
}

#[derive(Debug, Parsable, Clone)]
pub enum JSONNode {
	List(JSONList),
	Object(JSONObject),
	Value(Value)
}

fn main() {
	let file = fs::read_to_string("examples/json/example.json")
		.expect("Expected example file to exist.");

	let mut buffer = CharStream::new(file).build();
	let value = JSONNode::parse(&mut buffer);
	println!("value: {:#?}", value);
}
