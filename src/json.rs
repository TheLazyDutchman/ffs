use parse_macro_derive::Parsable;
use crate::parsing::{self, Group, List, tokens::{Bracket, Comma}, Parse};

#[derive(Parsable)]
pub struct JSONList {
	list: Group<Bracket, List<JSONNode, Comma>>
}

#[derive(Parsable)]
pub enum JSONNode {
	List(JSONList)
}