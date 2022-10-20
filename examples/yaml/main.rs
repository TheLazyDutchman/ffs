use std::fs;

use parseal::{parsing::{self, charstream::CharStream, Parse, tokens, Indent, Identifier}, Parsable, data_formats::{TreeData, ParseNode, NamedValue, Node}};

#[derive(Clone, Parsable, Debug)]
pub struct ListPart {
	token: tokens::Hyphen,
	value: YAMLNode
}

impl From<ListPart> for YAMLNode {
    fn from(part: ListPart) -> Self {
        part.value.clone()
    }
}

impl From<ListPart> for ParseNode<<YAMLNode as TreeData>::Object, <YAMLNode as TreeData>::List> {
    fn from(value: ListPart) -> Self {
        value.into()
    }
}

#[derive(Parsable, Debug, Clone)]
pub struct YAMLNode {
	value: ParseNode<<YAMLNode as TreeData>::Object, <YAMLNode as TreeData>::List>
}

impl TreeData for YAMLNode {
    type Object = Indent<NamedValue<Identifier, tokens::Colon, YAMLNode>>;

    type List = Indent<ListPart>;

    fn value(&self) -> ParseNode<Self::Object, Self::List> {
        self.value.clone()
    }
}

#[derive(Debug, Clone, Parsable)]
pub struct YAML {
	value: YAMLNode
}

pub fn main () {
	let file = fs::read_to_string("examples/yaml/example.yaml").unwrap();

	let mut charstream = CharStream::new(file).build();
	let value = YAML::parse(&mut charstream).unwrap();

	let node: Node = value.value.clone().into();
	println!("value: {:#?}", value);
	println!("value: {:#?}", node);
}