#![allow(unused)]

use std::fs;

use parse_macro_derive::Parsable;
use ffs::parsing::{tokens::{Less, Greater, Equal, ForwardSlash}, Parse, self, Identifier, StringValue, charstream::CharStream};

#[derive(Parsable)]
pub struct HTML {
    parts: Vec<Scope>
}

#[derive(Parsable)]
pub struct LabelArg(Identifier, Equal, StringValue);

#[derive(Parsable)]
pub struct LabelArgs {
    args: Vec<LabelArg>
}

#[derive(Parsable)]
pub struct StartLabel(Less, Identifier, Vec<LabelArgs>, Greater);

#[derive(Parsable)]
pub struct EndLabel(Less, ForwardSlash, Identifier, Greater);

#[derive(Parsable)]
pub struct Scope {
    start: StartLabel,
    end: EndLabel
}

fn main() {
    let file = fs::read_to_string("examples/html/example.html")
        .expect("Expected example file to exists.");

    let mut buffer = CharStream::new(file).build();

    HTML::parse(&mut buffer).unwrap();
}
