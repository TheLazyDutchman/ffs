#![allow(unused)]

use std::{collections::HashMap, fs};

use parseal::{
    data_formats::{NamedValue, Node, ParseNode, TreeData},
    parsing::{
        self,
        charstream::CharStream,
        tokens::{Brace, Bracket, Colon, Comma},
        Group, Identifier, List, Number, Parse, StringValue,
    },
    Parsable,
};

#[derive(Clone, Parsable, Debug)]
pub struct JSON {
    value: ParseNode<<JSON as TreeData>::Object, <JSON as TreeData>::List>,
}

impl TreeData for JSON {
    type Object = Group<Brace, List<NamedValue<StringValue, Colon, JSON>, Comma>>;

    type List = Group<Bracket, List<JSON, Comma>>;

    fn value(&self) -> ParseNode<Self::Object, Self::List> {
        self.value.clone()
    }
}

fn main() {
    let file =
        fs::read_to_string("examples/json/example.json").expect("Expected example file to exist.");

    let mut buffer = CharStream::new(file).build();
    let value = JSON::parse(&mut buffer).unwrap();

    let node: Node = value.clone().into();
    println!("value: {:#?}", value);
    println!("node: {:#?}", node);
}
