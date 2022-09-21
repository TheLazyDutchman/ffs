use self::tokens::Delimiter;

pub mod tokens;

pub struct Group<D, I> {
	delimiter: D,
	item: I
}

impl<D, I> Parse for Group<D, I> where
	D: Delimiter,
	I: Parse
{
    fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized {
		let start = D::Start::parse(value)?;
		let end = D::End::parse(value)?;
		let delimiter = D::new(start, end);

		let item = I::parse(value)?;
		Ok(Self { delimiter, item })
    }
}

pub struct List<I, S> {
	items: Vec<(I, Option<S>)>
}

impl<I, S> Parse for List<I, S> where
	I: Parse,
	S: tokens::Token
{
    fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized {
        let items = Vec::new();

		Ok(Self { items })
    }
}

pub trait Parse {
	fn parse(value: &str) -> Result<Self, ParseError> where Self: Sized;
}

pub struct ParseError {
	cause: String
}

impl ParseError {
	pub fn new(cause: &str) -> ParseError {
		ParseError { cause: cause.to_owned() }
	}
}