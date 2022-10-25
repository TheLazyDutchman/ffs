use parseal::{
    Parsable,
    parsing::{self, Parse, Identifier, Group, List, tokens},
    language_formats::{Define, TypeData},
};

use super::{attribute::Attribute, publicity::Pub};

#[derive(Debug, Parsable, Clone)]
pub struct TypeReference {
    name: Identifier,
    generics: Option<Box<Group<tokens::Chevron, List<TypeReference>>>>
}

#[derive(Debug, Parsable, Clone)]
pub struct NamedField {
    name: Identifier,
    colon: tokens::Colon,
    ty: TypeReference,
}

#[derive(Debug, Parsable, Clone)]
pub struct NamedFields(List<NamedField>);

#[derive(Debug, Parsable, Clone)]
pub struct StructData {
    #[value("struct")]
    keyword: Identifier,
    name: Identifier,
    value: Group<tokens::Brace, NamedFields>,
}

#[derive(Debug, Parsable, Clone)]
pub struct EnumVariant {
    name: Identifier
}

#[derive(Debug, Parsable, Clone)]
pub struct EnumData {
    #[value("enum")]
    keyword: Identifier,
    name: Identifier,
    data: Group<tokens::Brace, List<EnumVariant>>,
}

#[derive(Debug, Parsable, Clone)]
pub enum TypeObjectData {
    Struct(StructData),
    Enum(EnumData)
}

#[derive(Debug, Parsable, Clone)]
pub struct Type {
    attrs: Option<Vec<Attribute>>,
    vis: Pub,
    data: TypeObjectData,
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
