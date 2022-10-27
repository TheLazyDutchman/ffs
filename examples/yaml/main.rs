use std::fs;

use parseal::{
    data_formats::{NamedValue, Node, ParseNode, TreeData},
    parsing::{self, bufferstream::BufferStream, tokens, Identifier, Indent, Parse},
    Parsable,
};

#[derive(Clone, Parsable, Debug)]
pub struct ListPart {
    token: tokens::Hyphen,
    value: YAMLNode,
}

impl From<ListPart> for ParseNode<<YAMLNode as TreeData>::Object, <YAMLNode as TreeData>::List> {
    fn from(value: ListPart) -> Self {
        value.value.into()
    }
}

#[derive(Parsable, Debug, Clone)]
pub struct YAMLNode {
    value: ParseNode<<YAMLNode as TreeData>::Object, <YAMLNode as TreeData>::List>,
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
    start: [tokens::Hyphen; 3],
    value: YAMLNode,
}

pub fn main() {
    let file = fs::read_to_string("examples/yaml/example.yaml").unwrap();
    let mut buffer: BufferStream = file.into();
    let value = YAML::parse(&mut buffer).unwrap();

    println!("value: {:#?}", value);
    let node: Node = value.value.into();
    println!("value: {:#?}", node);
}
