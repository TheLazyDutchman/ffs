use super::{Parse, ParseError, charstream::CharStream};

pub trait Token: Parse {

}

pub trait Delimiter {
	type Start: Token;
	type End: Token;

	fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized;
}

macro_rules! create_tokens {
    ($($token:tt $id:ident),+) => {
        $(
            pub struct $id;
            
            impl Token for $id {}
            
            impl Parse for $id {
                fn parse(value: &mut CharStream<'_>) -> Result<Self, ParseError> {
                    let token = stringify!($token);
                    let len = token.len();

                    let mut token_value = value.clone();

                    let mut mtch = String::new();
                    while mtch.len() < len {
                        mtch.push(match token_value.next() {
                            Some(value) if value.is_whitespace() => continue,
                            Some(value) => value,
                            None => break
                        });
                    }

                    if (token == mtch) {
                        value.goto(token_value.position());
                        return Ok(Self {});
                    }

                    Err(ParseError::not_found(concat!("Could not find token '", stringify!($token), "'."), value.position()))
                }
            }
        )+
    };
}

macro_rules! create_delimiters {
    ($($token:tt $left: ident $right: ident $delim:ident),+) => {
        $(
            pub struct $left;

            impl Token for $left {}

            impl Parse for $left {
                fn parse(value: &mut CharStream<'_>) -> Result<Self, ParseError> {
                    let chr = stringify!($token).chars().nth(0).unwrap();

                    loop {
                        match value.peek() {
                            Some(peeked) if *peeked == chr => {
                                value.next();
                                return Ok(Self {})
                            }
                            Some(peeked) if peeked.is_whitespace() => {
                                value.next();
                            }
                            _ => break 
                        };
                    }
                    Err(ParseError::not_found(concat!("could not find left side of: '", stringify!($token), "'."), value.position()))
                }
            }

            pub struct $right;

            impl Token for $right {}

            impl Parse for $right {
                fn parse(value: &mut CharStream<'_>) -> Result<Self, ParseError> {
                    let chr = stringify!($token).chars().nth(1).unwrap();
                    loop {
                        match value.peek() {
                            Some(peeked) if *peeked == chr => {
                                value.next();
                                return Ok(Self {})
                            }
                            Some(peeked) if peeked.is_whitespace() => {
                                value.next();
                            }
                            _ => break
                        }
                    }
                    Err(ParseError::not_found(concat!("could not parse right side of: '", stringify!($token), "'."), value.position()))
                }
            }

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
            }
        )+
    };
}

create_tokens! {
    , Comma,
    . Period,
    ! Bang,
    # Hash,
    = Equal,
    == EqualEqual,
    : Colon,
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
