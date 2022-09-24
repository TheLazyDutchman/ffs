use std::fs;

use parse_macro_derive::Parsable;
use ffs::parsing::{self, tokens::{Identifier, Less, Greater, ForwardSlash}, Parse};

#[derive(Parsable)]
pub struct HTML {
}

#[derive(Parsable)]
pub struct LabelArgs {}

#[derive(Parsable)]
pub struct StartLabel(Less, Identifier, Vec<LabelArgs>, Greater);

#[derive(Parsable)]
pub struct EndLabel(Less, ForwardSlash, Identifier, Greater);

fn main() {
    let file = fs::read_to_string("examples/html/example.html")
        .expect("Expected example file to exists.");

    HTML::parse(&file).unwrap();
}
