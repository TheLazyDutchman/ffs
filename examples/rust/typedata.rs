use parseal::{Parsable, parsing::{self, Parse, Identifier, Group, tokens, List}};

use crate::{Visibility, Attribute};

#[derive(Parsable, Clone, Debug)]
pub struct TypePathReference {
    path: List<Identifier, tokens::DoubleColon>,
    generics: Option<Group<tokens::Chevron, List<Box<TypeValue>>>>,
}

#[derive(Parsable, Clone, Debug)]
pub enum TypeValue {
    Path(TypePathReference),
    Tuple(Group<tokens::Paren, List<Box<TypeValue>>>),
}

#[derive(Parsable, Clone, Debug)]
pub enum Variant {
    Tuple(Identifier, Group<tokens::Paren, List<UnnamedField>>),
    Unit(Identifier),
}

#[derive(Parsable, Clone, Debug)]
pub struct Enum {
    attrs: Option<Vec<Attribute>>,
    vis: Visibility,
    #[value("enum")]
    keyword: Identifier,
    name: Identifier,
    variants: Group<tokens::Brace, List<Variant>>,
}

#[derive(Parsable, Clone, Debug)]
pub struct UnnamedField {
    attrs: Option<Vec<Attribute>>,
    ty: TypeValue
}

#[derive(Parsable, Clone, Debug)]
pub struct NamedField {
    attrs: Option<Vec<Attribute>>,
    name: Identifier,
    colon: tokens::Colon,
    ty: TypeValue
}

#[derive(Parsable, Clone, Debug)]
pub enum StructData {
    Named(Group<tokens::Brace, List<NamedField>>),
}

impl StructData {
	fn fields(&self) -> Vec<NamedField> {
		match self {
			Self::Named(group) => group.clone().into()
		}
	}
}

#[derive(Parsable, Clone, Debug)]
pub struct Struct {
    attrs: Option<Vec<Attribute>>,
    vis: Visibility,
    #[value("struct")]
    keyword: Identifier,
    name: Identifier,
    data: StructData,
}