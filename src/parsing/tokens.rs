use super::{Parse, ParseError};

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
                fn parse(value: &str) -> Result<Self, ParseError> {
                    if value.starts_with(stringify!($token)) {
                        return Ok(Self {});
                    }

                    Err(ParseError::new(concat!("Could not find token '", stringify!($token), "'.")))
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
                fn parse(value: &str) -> Result<Self, ParseError> {
                    if value.starts_with(&stringify!($token)[..1]) {
                        return Ok(Self {});
                    }

                    Err(ParseError::new(concat!("could not find left side of: '", stringify!($token), "'.")))
                }
            }

            pub struct $right;

            impl Token for $right {}

            impl Parse for $right {
                fn parse(value: &str) -> Result<Self, ParseError> {
                    if value.starts_with(&stringify!($token)[1..]) {
                        return Ok(Self {});
                    }

                    Err(ParseError::new(concat!("could not parse right side of: '", stringify!($token), "'.")))
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
    : Colon,
    < Less,
    > Greater,
    / ForwardSlash
}

create_delimiters! {
    () LeftParen RightParen Paren,
    {} LeftBrace RightBrace Brace,
    [] LeftBracket RightBracket Bracket,
    "" Quote _LQuote Quotes
}
