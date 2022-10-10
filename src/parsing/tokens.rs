use super::{Parse, ParseError, charstream::CharStream};

pub trait Token: Parse {

}

pub trait Delimiter {
	type Start: Token;
	type End: Token;

	fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized;
    fn span(&self, value: &CharStream) -> super::Span;
}

macro_rules! create_tokens {
    ($($token:tt $id:ident),+) => {
        $(
            #[derive(Debug)]
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
                            Some(value) if value.is_whitespace() => continue,
                            Some(value) => value,
                            None => break
                        });
                    }

                    if (token == mtch) {
                        value.goto(token_value.position())?;
                        let end = value.position();
                        return Ok(Self { span: super::Span::new(start, end)});
                    }

                    Err(ParseError::not_found(concat!("Could not find token '", stringify!($token), "'."), token_value.position()))
                }

                fn span(&self, _: &CharStream) -> super::Span {
                    self.span.clone()
                }
            }
        )+
    };
}

macro_rules! create_delimiters {
    ($($token:tt $left: ident $right: ident $delim:ident),+) => {
        $(
            #[derive(Debug)]
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

                    Err(ParseError::not_found(concat!("could not find left side of: '", stringify!($token), "'."), value.position()))
                }

                fn span(&self, _: &CharStream) -> super::Span {
                    self.span.clone()
                }
            }

            #[derive(Debug)]
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
                        println!("end {}", chr);
                        if token == chr {
                            value.goto(token_value.position())?;
                            let end = value.position();
                            return Ok(Self { span: super::Span::new(start, end)})
                        }
                    }

                    Err(ParseError::not_found(concat!("could not find right side of: '", stringify!($token), "'."), value.position()))
                }

                fn span(&self, _: &CharStream) -> super::Span {
                    self.span.clone()
                }
            }

            #[derive(Debug)]
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

                fn span(&self, value: &CharStream) -> super::Span {
                    super::Span::new(self.start.span(value).start, self.end.span(value).end)
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
