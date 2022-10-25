use std::fs;

use parseal::{
    Parsable,
    parsing::{self, Parse, charstream::CharStream, tokens, Group, List, Identifier},
};

#[derive(Debug, Clone, Parsable)]
pub enum UsePart {
    Group(Group<tokens::Brace, List<Box<UsePart>>>),
    Path(Identifier, Option<(tokens::DoubleColon, Box<UsePart>)>),
}

#[derive(Parsable, Clone, Debug)]
pub struct TypePathReference {
    path: List<Identifier, tokens::DoubleColon>,
    generics: Option<Group<tokens::Chevron, List<Box<TypeReference>>>>,
}

#[derive(Parsable, Clone, Debug)]
pub enum TypeReference {
    Path(TypePathReference),
    Tuple(Group<tokens::Paren, List<Box<TypeReference>>>),
}

#[derive(Parsable, Clone, Debug)]
pub struct Attribute {
    start: tokens::Hash,
    value: Group<tokens::Bracket, (Identifier, Group<tokens::Paren, List<Identifier>>)>
}

#[derive(Parsable, Clone, Debug)]
pub enum Visibility {
    Public(#[value("pub")] Identifier),
    Private,
}

#[derive(Parsable, Clone, Debug)]
pub enum Variant {
    Tuple(Identifier, Group<tokens::Paren, List<TypeReference>>),
}

#[derive(Parsable, Clone, Debug)]
pub struct Enum {
    attrs: Option<Vec<Attribute>>,
    vis: Visibility,
    #[value("enum")]
    keyword: Identifier,
    name: Identifier,
    variants: Group<tokens::Brace, List<Variant>>,
}

#[derive(Parsable, Clone, Debug)]
pub enum Definition {
    Use(#[value("use")] Identifier, UsePart, tokens::Semicolon),
    Enum(Enum),
}

#[derive(Parsable, Clone, Debug)]
pub struct Rust {
    definitions: Vec<Definition>
}

pub fn main() {
    let file = fs::read_to_string("examples/rust/main.rs")
        .expect("could not find file.");
    let mut buffer = CharStream::new(file).build();
    let value = Rust::parse(&mut buffer);
    println!("value: {:#?}", value);
}
