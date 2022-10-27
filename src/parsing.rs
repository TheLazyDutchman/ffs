pub mod bufferstream;
pub mod tokens;
pub mod tokenstream;

use std::{collections::HashMap, fmt, vec::IntoIter};

use self::tokenstream::{Position, Span, Token, TokenStream, TokenType, WhitespaceType};

pub trait Parse: Clone {
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized;
    fn span(&self) -> Span;
}

#[derive(Clone)]
pub struct ParseError(String, Position);

impl ParseError {
    pub fn new(cause: &str, pos: Position) -> Self {
        Self(cause.to_string(), pos)
    }

    pub fn pos(&self) -> Position {
        self.1.clone()
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:Error: '{}'", self.1, self.0)
    }
}

/// A Group represents a delimited item.
/// Group has two Generic types:
/// - `D` is the delimiter tokens around the item, it has to a type that implements [`tokens::Delimiter`].
/// - `I` is the type of item inside the delimiters, it has to implement [`Parse`].
/// ```
/// # use parseal::parsing::{charstream::CharStream, tokens, Group, StringValue, Number, List, Parse};
/// # fn main() {
///     let buffer = "(\"Hello, World\")".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = Group::<tokens::Paren, StringValue>::parse(&mut buffer);
///     assert!(value.is_ok());
///
///     let buffer = "[0, 1, 2]".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = Group::<tokens::Bracket, List<Number, tokens::Comma>>::parse(&mut buffer);
///     assert!(value.is_ok());
/// # }
/// ```
#[derive(Clone)]
pub struct Group<D, I>
where
    D: tokens::Delimiter,
    I: Parse,
{
    delimiter: D,
    item: I,
}

impl<D, I> Parse for Group<D, I>
where
    D: tokens::Delimiter,
    I: Parse,
{
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let start = D::Start::parse(value)?;
        let item = I::parse(value)?;
        let end = match D::End::parse(value) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        let delimiter = D::new(start, end);

        Ok(Self { delimiter, item })
    }

    fn span(&self) -> Span {
        self.delimiter.span()
    }
}

impl<D, I> fmt::Debug for Group<D, I>
where
    D: tokens::Delimiter,
    I: Parse + fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Group({:#?}, delim: {}, from {:?})",
            self.item,
            D::name(),
            self.span()
        )
    }
}

impl<T, D, I> From<Group<D, I>> for Vec<T>
where
    Vec<T>: From<I>,
    D: tokens::Delimiter,
    I: Parse,
{
    fn from(group: Group<D, I>) -> Self {
        group.item.into()
    }
}

impl<S, T, D, I> From<Group<D, I>> for HashMap<S, T>
where
    Vec<(S, T)>: From<I>,
    D: tokens::Delimiter,
    I: Parse,
    S: std::cmp::Eq + std::hash::Hash,
{
    fn from(group: Group<D, I>) -> Self {
        let mut map = HashMap::new();
        map.extend::<Vec<_>>(group.item.into());
        map
    }
}

impl<
        D: tokens::Delimiter,
        T: Parse + IntoIterator<Item = I, IntoIter = Iter>,
        I,
        Iter: Iterator<Item = I>,
    > IntoIterator for Group<D, T>
{
    type Item = I;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.item.into_iter()
    }
}

/// A List represents a collection of items, separated by a token.
/// It has two generic types:
/// - `I` is the type of item, it has to implement [`Parse`].
/// - `S` is the token that separates the items. it has to implement [`tokens::Token`].
/// ```
/// # use parseal::parsing::{charstream::CharStream, tokens, Group, StringValue, Number, List, Parse};
/// # fn main() {
///     let buffer = "0, 1, 5".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = List::<Number, tokens::Comma>::parse(&mut buffer);
///     assert!(value.is_ok());
///
///     let buffer = "".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = List::<StringValue, tokens::Pipe>::parse(&mut buffer);
///     assert!(value.is_ok());
///     // A List can also be empty.
///
///     let buffer = "1012".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = List::<StringValue, tokens::Pipe>::parse(&mut buffer);
///     assert!(value.is_ok());
///     // the parse function is not guaranteed to consume the entire buffer.
///     // in this case it will not consume anything from the buffer, yet return an Ok variant, as the List is allowed to be empty.
/// # }
/// ```
#[derive(Clone)]
pub struct List<I, S = tokens::Comma>
where
    I: Parse,
    S: tokens::Token,
{
    items: Vec<(I, S)>,
    last_item: Option<I>,
    span: Span,
}

impl<I, S> Parse for List<I, S>
where
    I: Parse,
    S: tokens::Token,
{
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut items = Vec::new();
        let start = value.pos();
        let mut last_item = None;

        loop {
            let item = match I::parse(value) {
                Ok(value) => value,
                _ => break,
            };

            let separator = match S::parse(value) {
                Ok(value) => value,
                _ => {
                    last_item = Some(item);
                    break;
                }
            };

            items.push((item, separator));
        }

        let end = value.pos();

        Ok(Self {
            items,
            last_item,
            span: Span::new(start, end),
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<I, S> fmt::Debug for List<I, S>
where
    I: Parse + fmt::Debug,
    S: tokens::Token + fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items = self.items.iter().map(|(item, _)| item).collect::<Vec<_>>();
        match &self.last_item {
            Some(item) => items.push(item),
            None => {}
        }

        write!(f, "List({:#?}, from {:?})", items, self.span())
    }
}

impl<T, I, S> From<List<I, S>> for Vec<T>
where
    T: From<I>,
    I: Parse,
    S: tokens::Token,
{
    fn from(list: List<I, S>) -> Self {
        list.items
            .iter()
            .map(|(item, _)| item.clone().into())
            .collect()
    }
}

impl<Item: Parse, S: tokens::Token> IntoIterator for List<Item, S> {
    type Item = Item;
    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        let list: Vec<_> = self.into();
        list.into_iter()
    }
}

/// StringValue represents a string.
/// this is necessary because it needs to store some additional information for the AST, like the info necessary for [`Parse::span`].
/// ```
/// # use parseal::parsing::{StringValue, Parse, charstream::CharStream};
/// # fn main() {
///     let mut buffer = CharStream::new("\"Hello, world!\"".to_owned()).build();
///     let value = StringValue::parse(&mut buffer);
///
///     assert!(value.is_ok());
/// # }
/// ```
#[derive(Clone)]
pub struct StringValue {
    value: String,
    span: Span,
}

impl From<StringValue> for String {
    fn from(string: StringValue) -> Self {
        string.value
    }
}

impl Parse for StringValue {
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        match value.next() {
            Some(Token {
                value,
                span,
                tokentype: TokenType::String,
            }) => Ok(StringValue { value, span }),
            token => Err(ParseError::new(
                &format!("Expected string literal, got {:?}", token),
                value.pos(),
            )),
        }
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Debug for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringValue({}, from {:?})", self.value, self.span())
    }
}

/// An Identifier represents things like words and names.
/// ```
/// # use parseal::parsing::{charstream::CharStream, Identifier, Parse, tokens, self};
///
/// # fn main() {
///     let buffer = "hello world".to_owned();
///     let mut buffer = CharStream::new(buffer).build();
///
///     let value = Vec::<Identifier>::parse(&mut buffer).unwrap();
///     assert_eq!(value.len(), 2);
///
///     #[cfg(feature="derive")]
///     {
///         # use parseal::Parsable;
///         #[derive(Parsable, Clone)]
///         enum Bool {
///             True(#[value("true")] Identifier),
///             False(#[value("false")] Identifier)
///         }
///
///         let mut buffer = CharStream::new("true | false".to_owned()).build();
///         let value = <(Bool, tokens::Pipe, Bool)>::parse(&mut buffer);
///         assert!(value.is_ok());
///     }
/// # }
/// ```
#[derive(Clone)]
pub struct Identifier {
    identifier: String,
    span: Span,
}

impl Identifier {
    pub fn name(&self) -> String {
        self.identifier.clone()
    }
}

impl Parse for Identifier {
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        match value.next() {
            Some(Token {
                value: identifier,
                span,
                tokentype: TokenType::Identifier,
            }) => Ok(Identifier { identifier, span }),
            token => Err(ParseError::new(
                &format!("Expected identifier, got {:?}", token),
                value.pos(),
            )),
        }
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl From<Identifier> for String {
    fn from(ident: Identifier) -> Self {
        ident.identifier
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier({}, from {:?})", self.identifier, self.span)
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        &self.identifier == other
    }
}

/// A Number is a representation of a number, duh.
/// this representation is needed since it needs to store some additional information for the AST.
/// ```
/// # use parseal::parsing::{Number, Parse, charstream::CharStream};
/// # fn main() {
///     let mut buffer = CharStream::new("69420".to_owned()).build();
///     let value = Number::parse(&mut buffer);
///
///     assert!(value.is_ok());
/// # }
/// ```
#[derive(Clone)]
pub struct Number {
    value: String,
    span: Span,
}

impl From<Number> for usize {
    fn from(number: Number) -> Self {
        number.value.parse().unwrap()
    }
}

impl Parse for Number {
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        match value.next() {
            Some(Token {
                value,
                span,
                tokentype: TokenType::Identifier,
            }) => Ok(Number { value, span }),
            token => Err(ParseError::new(
                &format!("Expected number literal, got {:?}", token),
                value.pos(),
            )),
        }
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number({}, from {:?})", self.value, self.span)
    }
}

#[derive(Clone)]
pub struct Indent<T> {
    values: Vec<T>,
    depth: usize,
}

impl<T: fmt::Debug> Parse for Indent<T>
where
    T: Parse,
{
    fn parse<S: TokenStream>(value: &mut S) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut values = Vec::new();

        let mut indent_value = value.clone();
        indent_value.set_whitespace(WhitespaceType::Ignore);
        let mut pos = indent_value.pos();

        let mut item = T::parse(&mut indent_value);
        let depth = indent_value.indent();
        while item.is_ok() {
            pos = indent_value.pos();
            values.push(item?);
            item = T::parse(&mut indent_value);

            if indent_value.indent() != depth {
                break;
            }
        }

        if values.is_empty() {
            Err(ParseError("Could not find Indent block.".to_string(), pos))
        } else {
            Ok(Self { values, depth })
        }
    }

    fn span(&self) -> Span {
        Span::new(
            self.values.first().unwrap().span().start,
            self.values.last().unwrap().span().end,
        )
    }
}

impl<T> fmt::Debug for Indent<T>
where
    T: fmt::Debug + Parse,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Indent({:#?}, from {:?}, depth {})",
            self.values,
            self.span(),
            self.depth
        )
    }
}

impl<T, I> From<Indent<I>> for Vec<T>
where
    T: From<I>,
    I: Clone,
{
    fn from(indent: Indent<I>) -> Self {
        indent
            .values
            .iter()
            .map(|item| item.clone().into())
            .collect()
    }
}

#[cfg(feature = "data-formats")]
impl<T, I, S> From<Indent<I>> for HashMap<S, T>
where
    (S, T): From<I>,
    I: Clone,
    S: std::cmp::Eq + std::hash::Hash,
{
    fn from(indent: Indent<I>) -> Self {
        let mut map = HashMap::new();
        map.extend::<Vec<(S, T)>>(indent.into());
        map
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse + fmt::Debug,
{
    fn parse<S: TokenStream>(value: &mut S) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();

        let mut item = T::parse(value);
        while item.is_ok() {
            vec.push(item?);
            item = T::parse(value);
        }

        if vec.is_empty() {
            Err(ParseError(
                format!("Could not find vector because: {:#?}", item.unwrap_err()),
                value.pos(),
            ))
        } else {
            Ok(vec)
        }
    }

    fn span(&self) -> Span {
        Span::new(
            self.first().unwrap().span().start,
            self.last().unwrap().span().start,
        )
    }
}

impl<T, const N: usize> Parse for [T; N]
where
    T: Parse + fmt::Debug,
{
    fn parse<S: TokenStream>(value: &mut S) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut result = Vec::new();

        for _ in 0..N {
            result.push(T::parse(value)?);
        }

        match <[T; N]>::try_from(result) {
            Ok(result) => Ok(result),
            Err(error) => Err(ParseError(
                format!(
                    "Could not create slice from parsed values. \nvalues where: {:?}",
                    error
                ),
                value.pos(),
            )),
        }
    }

    fn span(&self) -> Span {
        Span::new(self[0].span().start, self[N - 1].span().end)
    }
}

//TODO: see if this can be more general
impl<A, B> Parse for (A, B)
where
    A: Parse,
    B: Parse,
{
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok((A::parse(value)?, B::parse(value)?))
    }

    fn span(&self) -> Span {
        Span::new(self.0.span().start, self.1.span().end)
    }
}

impl<A, B, C> Parse for (A, B, C)
where
    A: Parse,
    B: Parse,
    C: Parse,
{
    fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok((A::parse(value)?, B::parse(value)?, C::parse(value)?))
    }

    fn span(&self) -> Span {
        Span::new(self.0.span().start, self.2.span().end)
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse<S: TokenStream>(value: &mut S) -> Result<Self, ParseError> {
        let mut __value = value.clone();
        match T::parse(&mut __value) {
            Ok(result) => {
                value.goto(__value.pos())?;
                Ok(Some(result))
            }
            Err(_) => Ok(None),
        }
    }

    /// TODO deal with the None case, currently the outside caller has to check for it.
    fn span(&self) -> Span {
        Span::default()
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse<S: TokenStream>(value: &mut S) -> Result<Self, ParseError> {
        Ok(Box::new(T::parse(value)?))
    }

    fn span(&self) -> Span {
        self.as_ref().span()
    }
}
