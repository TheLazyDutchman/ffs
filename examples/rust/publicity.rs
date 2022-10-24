use parseal::{
    Parsable,
    parsing::{Parse, Identifier, self},
};

#[derive(Debug, Clone, Parsable)]
pub enum PubType {
    Pub(#[value("pub")] Identifier),
}

#[derive(Debug, Clone, Parsable)]
pub struct Pub {
    value: Option<PubType>
}
