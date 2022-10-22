use std::{fs, fmt::{self, Formatter}};

use parseal::{
    language_formats::{Definition, LanguageData},
    parsing::{self, tokens, charstream::CharStream, Parse},
    Parsable,
};

#[derive(Parsable, Clone)]
pub struct DoubleColon([tokens::Colon; 2]);

impl tokens::Token for DoubleColon {}

impl fmt::Display for DoubleColon {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "::")
    }
}

impl fmt::Debug for DoubleColon {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Token(::, from {})", self.span())
    }
}

#[derive(Parsable, Debug, Clone)]
pub struct Rust {
    definitions: Vec<Definition<Rust>>,
}

mod function;
mod import;
mod variable;

use function::Function;
use import::Import;
use variable::Variable;

impl LanguageData for Rust {
    type Function = Function;
    type Import = Import;
    type Variable = Variable;
}

pub fn main() {
    let file = fs::read_to_string("examples/rust/main.rs").expect("Can't read file");
    let mut buffer = CharStream::new(file).build();
    let value = Rust::parse(&mut buffer).unwrap();
    println!("value: {:#?}", value);
}
