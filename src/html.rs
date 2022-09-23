use parse_macro_derive::Parsable;
use super::parsing::{self, tokens::{Identifier, Less, Greater, ForwardSlash}};

#[derive(Parsable)]
pub struct HTML {
}

#[derive(Parsable)]
pub struct LabelArgs {}

#[derive(Parsable)]
pub struct StartLabel(Less, Identifier, Vec<LabelArgs>, Greater);

#[derive(Parsable)]
pub struct EndLabel(Less, ForwardSlash, Identifier, Greater);
