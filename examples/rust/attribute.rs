use parseal::{
    Parsable,
    parsing::{Parse, Identifier, tokens, Group, self, List},
};

#[derive(Debug, Clone, Parsable)]
pub enum AttributeValue {
    List(Identifier, Group<tokens::Paren, List<Identifier>>), // group should not actually contain Identifier
}

#[derive(Debug, Clone, Parsable)]
pub struct Attribute {
    token: tokens::Hash,
    value: Group<tokens::Bracket, AttributeValue>,
}
