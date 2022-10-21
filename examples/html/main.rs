#![allow(unused)]

use std::fs;

use parseal::{
    parsing::{
        self,
        charstream::CharStream,
        tokens::{Equal, ForwardSlash, Greater, Less},
        Identifier, Parse, StringValue,
    },
    Parsable,
};

#[derive(Parsable, Debug, Clone)]
pub struct HTML {
    parts: Vec<Scope>,
}

#[derive(Parsable, Debug, Clone)]
pub struct LabelArg(Identifier, Equal, StringValue);

#[derive(Parsable, Debug, Clone)]
pub struct LabelArgs {
    args: Vec<LabelArg>,
}

#[derive(Parsable, Debug, Clone)]
pub struct StartLabel(Less, Identifier, Vec<LabelArgs>, Greater);

#[derive(Parsable, Debug, Clone)]
pub struct EndLabel(Less, ForwardSlash, Identifier, Greater);

#[derive(Parsable, Debug, Clone)]
pub struct Scope {
    start: StartLabel,
    end: EndLabel,
}

fn main() {
    let file =
        fs::read_to_string("examples/html/example.html").expect("Expected example file to exists.");

    let mut buffer = CharStream::new(file).build();

    HTML::parse(&mut buffer).unwrap();
}
