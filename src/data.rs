use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::parsing::{AST, token::Token, ParserError};

pub mod json;
pub mod yaml;

#[derive(Debug, PartialEq)]
pub enum Data {
    List(Vec<Data>),
    Object(HashMap<String, Data>),
    Immediate(Token)
}

pub trait TreeData : AST {
    fn parse_data(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError>;
    fn parse_list(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError>;
    fn parse_object(tokens: &mut Peekable<Iter<Token>>) -> Result<Data, ParserError>;
}