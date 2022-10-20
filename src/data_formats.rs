use std::collections::HashMap;

use crate::{Parsable, parsing::{self, tokens, Parse, StringValue, Number, Identifier}};

#[derive(Clone, Parsable, Debug)]
pub struct NamedValue<S, I>(StringValue, S, I) where S: tokens::Token, I: Parse;

impl<T, S, I> From<NamedValue<S, I>> for (String, T) where
	T: From<I>,
	S: tokens::Token,
	I: Parse
{
	fn from(value: NamedValue<S, I>) -> Self {
		(value.0.into(), value.2.into())
	}
}

#[derive(Parsable, Clone, Debug)]
pub enum ParseValue {
	String(StringValue),
	Number(Number),
	Bool(#[value("true", "false")] Identifier)
}

#[derive(Parsable, Clone, Debug)]
pub enum ParseNode<Object, List> where 
	Object: Parse, 
	List: Parse,
	HashMap<String, ParseNode<Object, List>>: From<Object>,
	Vec<ParseNode<Object, List>>: From<List>
{
	Value(ParseValue),
	Object(Object),
	List(List)
}

#[derive(Debug)]
pub enum Value {
	String(String),
	Number(usize),
	Bool(bool)
}

impl From<ParseValue> for Value {
    fn from(value: ParseValue) -> Self {
        match value {
			ParseValue::String(value) => Self::String(value.into()),
			ParseValue::Number(value) => Self::Number(value.into()),
			ParseValue::Bool(value) => Self::Bool(value.name() == "true"),
		}
    }
}

#[derive(Debug)]
pub enum Node {
	Value(Value),
	Object(HashMap<String, Node>),
	List(Vec<Node>)
}

impl<Object, List> From<ParseNode<Object, List>> for Node where 
	Object: Parse,
	List: Parse,
	HashMap<String, ParseNode<Object, List>>: From<Object>,
	Vec<ParseNode<Object, List>>: From<List>
{
    fn from(value: ParseNode<Object, List>) -> Self {
        match value {
			ParseNode::Value(value) => Self::Value(value.into()),
			ParseNode::Object(value) => {
				let map: HashMap<String, ParseNode<Object, List>> = value.into();
				let map = map.iter().map(|(key, value)| {
					let node: Node = value.clone().into();
					(key.clone(), node)
			}).collect::<HashMap<String, Node>>();
				Self::Object(map)
			}
			ParseNode::List(value) => {
				let list: Vec<_> = value.into();
				let list = list.iter().map(|item| item.clone().into()).collect();
				Self::List(list)
			}
		}
    }
}
