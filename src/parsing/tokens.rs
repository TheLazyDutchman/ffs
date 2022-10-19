use std::fmt;

use super::{Parse, ParseError, charstream::CharStream};

pub trait Token: Parse + fmt::Display {

}

pub trait Delimiter: Clone {
	type Start: Token;
	type End: Token;

	fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized;
    fn span(&self) -> super::Span;
    fn name() -> String;
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
                fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
                    let token = stringify!($token);
                    let len = token.len();
                    let start = value.position();

                    let mut token_value = value.clone();

                    let mut mtch = String::new();
                    while mtch.len() < len {
                        mtch.push(match token_value.next() {
                            Some(value) => value,
                            None => break
                        });
                    }

                    if (token == mtch) {
                        value.goto(token_value.position())?;
                        let end = value.position();
                        return Ok(Self { span: super::Span::new(start, end)});
                    }

                    Err(ParseError(format!("Could not find token '{}'.", stringify!($token)), token_value.position()))
                }

                fn span(&self) -> super::Span {
                    self.span.clone()
                }
            }

            impl fmt::Debug for $id {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                    write!(f, "Token({}, at: {})", stringify!($token), self.span.end)
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
                fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
                    let chr = stringify!($token).chars().nth(0).unwrap();
                    let mut token_value = value.clone();
                    let start = value.position();

                    if let Some(token) = token_value.next() {
                        if token == chr {
                            value.goto(token_value.position())?;
                            let end = value.position();
                            return Ok(Self { span: super::Span::new(start, end)})
                        }
                    }

                    Err(ParseError(format!("could not find left side of: '{}'.", stringify!($token)), value.position()))
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
                fn parse(value: &mut CharStream) -> Result<Self, ParseError> where Self: Sized {
                    let chr = stringify!($token).chars().nth(1).unwrap();
                    let mut token_value = value.clone();
                    let start = value.position();

                    if let Some(token) = token_value.next() {
                        if token == chr {
                            value.goto(token_value.position())?;
                            let end = value.position();
                            return Ok(Self { span: super::Span::new(start, end)})
                        }
                    }

                    Err(ParseError(format!("could not find right side of: '{}'.", stringify!($token)), value.position()))
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
    _ UnderScore,
    - Hyphen,
    + Plus,
    = Equal,
    == EqualEqual,
    : Colon,
    ; Semicolon,
    | Pipe,
    < Less,
    > Greater,
    / ForwardSlash
}

create_delimiters! {
    () LeftParen RightParen Paren,
    {} LeftBrace RightBrace Brace,
    [] LeftBracket RightBracket Bracket,
    "" LeftQuote RightQuote Quote
}
