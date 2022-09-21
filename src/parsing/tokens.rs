use super::Parse;

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

pub struct Comma;

impl Token for Comma {

}

pub struct LeftBracket;

impl Token for LeftBracket {

}

pub struct RightBracket;

impl Token for RightBracket {

}

impl<T> Parse for T where T: Token {
    fn parse<E>(value: &str) -> Result<Self, E> {
        todo!()
    }
}