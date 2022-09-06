use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::parsing::{AST, token::Token, ParserError};

pub mod json;
pub mod yaml;
pub mod tsv;
pub mod csv;

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

#[derive(Debug, PartialEq)]
pub struct Row {
    pub values: Vec<Token>
}

pub trait GridData : AST {
    fn parse_data(tokens: &mut Peekable<Iter<Token>>) -> Result<Token, ParserError>;
    fn parse_row(tokens: &mut Peekable<Iter<Token>>) -> Result<Row, ParserError>;
}