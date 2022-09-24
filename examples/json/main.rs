use std::fs;

use parse_macro_derive::Parsable;
use ffs::parsing::{ParseError, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Number, StringValue, Parse};

#[derive(Parsable)]
pub struct JSONList {
	list: Group<Bracket,
		List<JSONNode, Comma>>
}

#[derive(Parsable)]
pub struct NamedValue {
	name: StringValue,
	colon: Colon,
	value: JSONNode
}

#[derive(Parsable)]
pub struct JSONObject {
	map: Group<Brace,
		List<NamedValue, Comma>>
}

#[derive(Parsable)]
pub enum Value {
	String(StringValue),
	Number(Number)
}

#[derive(Parsable)]
pub enum JSONNode {
	List(JSONList),
	Object(JSONObject),
	Value(Value)
}

fn main() {
	let file = fs::read_to_string("examples/json/example.json")
		.expect("Expected example file to exist.");

	JSONNode::parse(&file).unwrap();
}
