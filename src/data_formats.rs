use std::collections::HashMap;

use crate::{Parsable, parsing::{self, tokens, Parse, StringValue, Number, Identifier}};

#[derive(Clone, Parsable, Debug)]
pub struct NamedValue<N, S, I>(N, S, I) where N: Parse, S: tokens::Token, I: Parse;

impl<T, N, S, I> From<NamedValue<N, S, I>> for (String, T) where
	T: From<I>,
	N: Parse,
	String: From<N>,
	S: tokens::Token,
	I: Parse
{
	fn from(value: NamedValue<N, S, I>) -> Self {
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
	List: Parse
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

pub trait TreeData {
	type Object: Parse;
	type List: Parse;

	fn value(&self) -> ParseNode<Self::Object, Self::List>;
}

impl<
	Object: Parse, 
	List: Parse,
	T: TreeData<Object = Object, List = List>> From<T> for Node where
	HashMap<String, ParseNode<Object, List>>: From<Object>,
	Vec<ParseNode<Object, List>>: From<List>
{
	fn from(tree: T) -> Self {
		tree.value().into()
	}
}

impl<
	Object: Parse,
	List: Parse,
	T: TreeData<Object = Object, List = List>> From<T> 
for ParseNode<T::Object, T::List> {
    fn from(tree: T) -> Self {
        tree.value()
    }
}