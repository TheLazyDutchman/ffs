#![allow(unused)]

use std::fs;

use parseal::{
    Parsable,
    parsing::{self, Parse, charstream::CharStream, tokens, Group, List, Identifier, StringValue},
};
use typedata::{NamedField, Enum, Struct};

mod typedata;

#[derive(Debug, Clone, Parsable)]
pub enum UsePart {
    Group(Group<tokens::Brace, List<Box<UsePart>>>),
    Path(Identifier, Option<(tokens::DoubleColon, Box<UsePart>)>),
}

#[derive(Parsable, Clone, Debug)]
pub enum AttrValue {
    Ident(Identifier),
    String(StringValue),
}

#[derive(Parsable, Clone, Debug)]
pub struct Attribute {
    start: tokens::Hash,
    outer: Option<tokens::Bang>,
    value: Group<tokens::Bracket, (Identifier, Group<tokens::Paren, List<AttrValue>>)>
}

#[derive(Parsable, Clone, Debug)]
pub enum Visibility {
    Public(#[value("pub")] Identifier),
    Private,
}

#[derive(Parsable, Clone, Debug)]
pub struct Function {
    attrs: Option<Vec<Attribute>>,
    vis: Visibility,
    #[value("fn")]
    keyword: Identifier,
    name: Identifier,
    parameters: Group<tokens::Paren, List<NamedField>>,
}

#[derive(Parsable, Clone, Debug)]
pub enum Definition {
    Use(#[value("use")] Identifier, UsePart, tokens::Semicolon),
    Mod(#[value("mod")] Identifier, Identifier, tokens::Semicolon),
    Enum(Enum),
    Struct(Struct),
    Function(Function),
}

#[derive(Parsable, Clone, Debug)]
pub struct Rust {
    attrs: Option<Vec<Attribute>>,
    definitions: Vec<Definition>
}

pub fn main() {
    let file = fs::read_to_string("examples/rust/main.rs")
        .expect("could not find file.");
    let mut buffer = CharStream::new(file).build();
    let value = Rust::parse(&mut buffer);
    println!("value: {:#?}", value);
}
