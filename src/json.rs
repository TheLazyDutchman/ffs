use parse_macro_derive::Parsable;
use crate::parsing::{self, Group, List, tokens::{Bracket, Comma, Brace, Colon}, Parse, Number};

#[derive(Parsable)]
pub struct JSONList {
	list: Group<Bracket,
		List<JSONNode, Comma>>
}

#[derive(Parsable)]
pub struct NamedValue {
	name: String,
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
	String(String),
	Number(Number)
}

#[derive(Parsable)]
pub enum JSONNode {
	List(JSONList),
	Object(JSONObject),
	Value(Value)
}