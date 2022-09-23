use super::{Parse, ParseError};

pub trait Token: Parse {

}

pub trait Delimiter {
	type Start: Token;
	type End: Token;

	fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized;
}

pub struct Bracket {
	start: <Self as Delimiter>::Start,
	end: <Self as Delimiter>::End
}

impl Delimiter for Bracket {
    type Start = LeftBracket;

    type End = RightBracket;

    fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized {
        Self { start, end }
    }
}

pub struct Brace {
    start: <Self as Delimiter>::Start,
    end: <Self as Delimiter>::End
}

impl Delimiter for Brace {
    type Start = LeftBrace;

    type End = RightBrace;

    fn new(start: Self::Start, end: Self::End) -> Self where Self: Sized {
        todo!()
    }
}

pub struct Comma;

impl Token for Comma {

}

pub struct Colon;

impl Token for Colon {

}

pub struct LeftBracket;

impl Token for LeftBracket {

}

pub struct RightBracket;

impl Token for RightBracket {

}

pub struct LeftBrace;

impl Token for LeftBrace {

}

pub struct RightBrace;

impl Token for RightBrace {

}

impl<T> Parse for T where T: Token {
    fn parse(value: &str) -> Result<Self, ParseError> {
        todo!()
    }
}