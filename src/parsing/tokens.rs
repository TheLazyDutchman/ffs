use std::fmt;

use super::{
    tokenstream::{self, TokenStream, TokenType, WhitespaceType, Span},
    Parse, ParseError,
};

pub trait Token: Parse + fmt::Display {}

pub trait Delimiter: Clone {
    type Start: Token;
    type End: Token;

    fn new(start: Self::Start, end: Self::End) -> Self
    where
        Self: Sized;
    fn span(&self) -> super::Span;
    fn name() -> String;
}

#[derive(Clone)]
pub struct Chevron {
    start: Less,
    end: Greater,
}

impl Delimiter for Chevron {
    type Start = Less;
    type End = Greater;

    fn new(start: Less, end: Greater) -> Self {
        Self { start, end }
    }

    fn span(&self) -> super::Span {
        super::Span::new(self.start.span().start, self.end.span().end)
    }

    fn name() -> String {
        "<>".to_string()
    }
}

macro_rules! create_tokens {
    ($($token:tt $id:ident),+) => {
        $(
            #[derive(Clone)]
            pub struct $id {
                span: super::Span
            }

            impl Token for $id {}

            impl Parse for $id {
                fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError> where Self: Sized {
                    let string = stringify!($token);
                    let mut __value = value.clone();
                    __value.set_whitespace(WhitespaceType::KeepAll);
                    let mut result = String::new();
                    let start = __value.pos();

                    loop {
                        match __value.next() {
                            Some(tokenstream::Token {
                                value,
                                span: _,
                                tokentype: TokenType::Token
                            }) => result.push_str(&value),
                            _ => break
                        }
                        if result.len() == string.len() {
                            break
                        }
                    }

                    if string == result {
                        let end = __value.pos();
                        value.goto(end.clone())?;
                        return Ok(Self { span: Span::new(start, end) } )
                    } else {
                        Err(ParseError::new("Did not find token '{}'.", value.pos()))
                    }
                }

                fn span(&self) -> super::Span {
                    self.span.clone()
                }
            }

            impl fmt::Debug for $id {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                    write!(f, "Token({}, at: {:?})", stringify!($token), self.span.end)
                }
            }

            impl fmt::Display for $id {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, "Token({})", stringify!($token))
                }
            }
        )+
    };
}

macro_rules! create_delimiters {
    ($($token:tt $left: ident $right: ident $delim:ident),+) => {
        $(
            #[derive(Clone)]
            pub struct $left {
                span: super::Span
            }

            impl Token for $left {}

            impl Parse for $left {
                fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError> where Self: Sized {
                    let chr = stringify!($token).chars().nth(0).unwrap();
                    match value.next() {
                        Some(tokenstream::Token {
                            value, span, tokentype: TokenType::Token
                        }) if value.chars().nth(0).unwrap() == chr => Ok(Self { span }),
                        token => Err(ParseError(format!("Expected left side of: '{}', got '{:?}'.", stringify!($token), token), value.pos()))
                    }
                }

                fn span(&self) -> super::Span {
                    self.span.clone()
                }
            }

            impl fmt::Debug for $left {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, "Token({})", stringify!($token).chars().nth(0).unwrap())
                }
            }

            impl fmt::Display for $left {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, "Token({})", stringify!($token).chars().nth(0).unwrap())
                }
            }

            #[derive(Clone)]
            pub struct $right {
                span: super::Span
            }

            impl Token for $right {}

            impl Parse for $right {
                fn parse<T: TokenStream>(value: &mut T) -> Result<Self, ParseError> where Self: Sized {
                    let chr = stringify!($token).chars().nth(1).unwrap();
                    match value.next() {
                        Some(tokenstream::Token {
                            value, span, tokentype: TokenType::Token
                        }) if value.chars().nth(0).unwrap() == chr => Ok(Self { span }),
                        token => {
                            println!("got: {:?}, expected: {}", token, chr);
                            Err(ParseError(format!("Expected right side of: '{}', got '{:?}'.", stringify!($token), token), value.pos()))
                        }
                    }
                }

                fn span(&self) -> super::Span {
                    self.span.clone()
                }
            }

            impl fmt::Debug for $right {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, "Token({})", stringify!($token).chars().nth(1).unwrap())
                }
            }

            impl fmt::Display for $right {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, "Token({})", stringify!($token).chars().nth(1).unwrap())
                }
            }

            #[derive(Debug, Clone)]
            pub struct $delim {
                start: $left,
                end: $right
            }

            impl Delimiter for $delim {
                type Start = $left;
                type End = $right;

                fn new(start: Self::Start, end: Self::End) -> Self {
                    Self { start, end }
                }

                fn span(&self) -> super::Span {
                    super::Span::new(self.start.span().start, self.end.span().end)
                }

                fn name() -> String {
                    String::from(stringify!($delim))
                }
            }
        )+
    };
}

create_tokens! {
    , Comma,
    . Period,
    ! Bang,
    # Hash,
    $ Dollar,
    % Percentage,
    ^ UpArrow,
    & Ampersand,
    * Asterisk,
    _ UnderScore,
    - Hyphen,
    + Plus,
    = Equal,
    == EqualEqual,
    : Colon,
    :: DoubleColon,
    ; Semicolon,
    | Pipe,
    < Less,
    > Greater,
    / ForwardSlash,
    -> RightArrow,
    => FatArrow
}

create_delimiters! {
    () LeftParen RightParen Paren,
    {} LeftBrace RightBrace Brace,
    [] LeftBracket RightBracket Bracket,
    "" LeftQuote RightQuote Quote
}
