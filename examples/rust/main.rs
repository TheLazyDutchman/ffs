use std::fs;

use parseal::{
    language_formats::LanguageData,
    parsing::{self, charstream::CharStream, Identifier, Parse},
    Parsable,
};

#[derive(Parsable, Debug, Clone)]
pub struct Rust {
    functions: Vec<Function>,
}

#[derive(Parsable, Debug, Clone)]
pub struct Function {
    #[value("fn")]
    keyword: Identifier,
}

impl LanguageData for Rust {
    type FunctionData = Function;
}

pub fn main() {
    let file = fs::read_to_string("examples/rust/main.rs").expect("Can't read file");
    let mut buffer = CharStream::new(file).build();
    let value = Rust::parse(&mut buffer).unwrap();
    println!("value: {:?}", value);
}
