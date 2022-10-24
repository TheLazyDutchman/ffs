use parseal::{
    Parsable,
    parsing::{self, Parse, Identifier},
    language_formats::{Define, TypeData},
};

use super::{attribute::Attribute, publicity::Pub};

#[derive(Debug, Parsable, Clone)]
pub struct Type {
    attrs: Vec<Attribute>,
    vis: Pub
}

impl Define<TypeData> for Type {
    fn name(&self) -> Identifier {
        todo!()
    }
}

impl Into<TypeData> for Type {
    fn into(self) -> TypeData {
        todo!()
    }
}
