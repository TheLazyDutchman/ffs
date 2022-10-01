#![allow(unused)]

use std::fs;

use parse_macro_derive::Parsable;
use ffs::parsing::{self, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Number, StringValue, Parse, charstream::CharStream};

#[derive(Debug, Parsable)]
pub struct JSONList {
	list: Group<Bracket,
		List<JSONNode, Comma>>
}

#[derive(Debug, Parsable)]
pub struct NamedValue {
	name: StringValue,
	colon: Colon,
	value: JSONNode
}

#[derive(Debug, Parsable)]
pub struct JSONObject {
	map: Group<Brace,
		List<NamedValue, Comma>>
}

#[derive(Debug, Parsable)]
pub enum Value {
	String(StringValue),
	Number(Number)
}

#[derive(Debug, Parsable)]
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
	println!("value: {:?}", value);
}
