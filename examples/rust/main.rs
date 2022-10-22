use std::fs;

use parseal::{
    language_formats::{LanguageData, Definition},
    parsing::{self, charstream::CharStream, Parse},
    Parsable,
};

#[derive(Parsable, Debug, Clone)]
pub struct Rust {
    functions: Vec<Definition<Rust>>,
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
    println!("value: {:?}", value);
}
