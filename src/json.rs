use parse_macro_derive::Parse;
use crate::parsing::{Group, List, tokens::{Brackets, Comma}};

use super::Parse;

#[derive(Parse)]
pub struct JSONList {
	list: Group<Brackets, List<JSONNode, Comma>>
}

pub enum JSONNode {

}