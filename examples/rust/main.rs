use std::fs;

use parseal::{
    language_formats::{Definition, LanguageData},
    parsing::{self, charstream::CharStream, Parse},
    Parsable,
};

#[derive(Parsable, Debug, Clone)]
pub struct Rust {
    definitions: Vec<Definition<Rust>>,
}

mod function;
mod import;
mod variable;
mod type_object;
mod attribute;
mod publicity;

use function::Function;
use import::Import;
use variable::Variable;
use type_object::Type;

impl LanguageData for Rust {
    type Function = Function;
    type Import = Import;
    type Variable = Variable;
    type Type = Type;
}

pub fn main() {
    let file = fs::read_to_string("examples/rust/main.rs").expect("Can't read file");
    let mut buffer = CharStream::new(file).build();
    let value = Rust::parse(&mut buffer).unwrap();
    println!("value: {:#?}", value);
}
