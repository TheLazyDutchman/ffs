use std::{error::Error, fmt::{Display, Debug}};
use std::fs;

pub struct Parser {
}

pub trait Node: Debug {}

#[derive(Debug)]
pub struct Token {
    value: char
}

impl Token {
    pub fn new(value: char) -> Self {
        Self { value }
    }
}

impl Node for Token {}

#[derive(Debug)]
pub struct AST {
    statements: Vec<Box<dyn Node>>
}

impl From<String> for AST {
    fn from(value: String) -> Self {
        let mut statements: Vec<Box<dyn Node>> = Vec::new();

        for chr in value.chars() {
            statements.push(Box::new(Token::new(chr)));
        }

        Self { statements }
    }
}

#[derive(Debug)]
pub struct ParserError {
    filename: String
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error when parsing file: {}", self.filename)
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, path: &str) -> Result<AST, ParserError> {
        let string = AST::from(fs::read_to_string(path).unwrap());
        println!("{:?}", string);
        todo!()
    }
}