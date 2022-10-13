use std::fs;

use ffs::parsing::{self, charstream::CharStream, Parse, StringValue, Number, Identifier, tokens};
use parse_macro_derive::Parsable;

#[derive(Parsable, Debug)]
pub enum NamedValue {
	Object(ObjectValue),
	String(StringValue),
	Number(Number),
	Bool(Identifier)
}

#[derive(Parsable, Debug)]
pub enum ObjectValue {
	Object(Vec<(Identifier, tokens::Colon, NamedValue)>),
	List(Vec<(tokens::Hyphen, Value)>)
}

#[derive(Parsable, Debug)]
pub enum Value {
	Named(Identifier, tokens::Colon, ObjectValue),
	String(StringValue),
	Number(Number),
	Bool(Identifier)
}

#[derive(Parsable, Debug)]
pub struct YAML {
	#[whitespace(KeepAll)]
	start: [tokens::Hyphen; 3],
	#[whitespace(KeepAll)]
	value: Value
}

pub fn main () {
	let file = fs::read_to_string("examples/yaml/example.yaml").unwrap();

	let mut charstream = CharStream::new(file).build();
	let value = YAML::parse(&mut charstream);

	println!("value: {:#?}", value);
}