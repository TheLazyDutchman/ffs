This crate defines the derive macro for the [parseal](https://www.github.com/TheLazyDutchman/parseal) crate.

For tuple structs, the generated code should look something like this:
```rust
#[derive(Parsable)]
struct Test(Number, Comma, Number);

// generated code
impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_0 = Number::parse(value)?;
		let __inner_1 = Comma::parse(value)?;
		let __inner_2 = Number::parse(value)?;
		Ok(Self(__inner_0, __inner_1, __inner_2))
	}

	fn span(&self) -> Span {
		Span::new(self.0.span().start, self.2.span().end)
	}
}

#[derive(Parsable)]
struct TestWhiteSpace(#[whitespace(KeepAll)] [tokens::Hyphen; 3], Number);

// generated code
impl Parse for TestWhiteSpace {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_0 = {
			let __white_space_value = value.clone();
			__white_space_value.set_whitespace(WhiteSpaceType::KeepAll);

			let inner = <[tokens::Hyphen; 3]>::parse(&mut __white_space_value);
			value.goto(__white_space_value.position())?;
			inner
		}?;
		let __inner_1 = Number::parse(value)?;
		Ok(Self(__inner_0, __inner_1))
	}

	fn span(&self) -> Span {
		Span::new(self.0.span().start, self.1.span().end)
	}
}

#[derive(Parsable)]
struct TestValue(#[value("test", "other")] Identifier, Number);

// generated code
impl Parse for TestWhiteSpace {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_0 = match Identifier::parse(value) {
			Ok(__inner_0) if __inner_0 == "test" => __inner_0,
			Ok(__inner_0) if __inner_0 == "other" => __inner_0,
			Ok(__inner_0) => return Err(ParseError("Value was not one of the expected values.", value.position())),
			Err(error) => return Err(error)
		};
		let __inner_1 = Number::parse(value)?;
		Ok(Self(__inner_0, __inner_1))
	}

	fn span(&self) -> Span {
		Span::new(self.0.span().start, self.1.span().end)
	}
}

#[derive(Parsable)]
struct TestValueWhiteSpace(#[whitespace(Indent)] #[value("test", "other")] Identifier, Number);

// generated code
impl Parse for TestWhiteSpace {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_0 = match {
			let __white_space_value = value.clone();
			__white_space_value.set_whitespace(WhiteSpaceType::Indent);

			let inner = Identifier::parse(&mut __white_space_value);
			value.goto(__white_space_value.position())?;
			inner
		} {
			Ok(__inner_0) if __inner_0 == "test" => __inner_0,
			Ok(__inner_0) if __inner_0 == "other" => __inner_0,
			Ok(__inner_0) => return Err(ParseError("Value was not one of the expected values.", value.position())),
			Err(error) => return Err(error)
		};
		let __inner_1 = Number::parse(value)?;
		Ok(Self(__inner_0, __inner_1))
	}

	fn span(&self) -> Span {
		Span::new(self.0.span().start, self.1.span().end)
	}
}
```

For named structs, it should look like this:
```rust
#[derive(Parsable)]
struct Test {
	x: Number,
	comma: Comma,
	y: Number
}

// generated code
impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_x = Number::parse(value)?;
		let __inner_comma = Comma::parse(value)?;
		let __inner_y = Number::parse(value)?;
		Ok(Self { x: __inner_x, comma: __inner_comma, y: __inner_y})
	}

	fn span(&self) -> Span {
		Span::new(self.x.span().start, self.y.span().end)
	}
}

#[derive(Parsable)]
struct TestWhiteSpace {
	#[whitespace(Indent)]
	x: Number,
	comma: Comma,
	y: Number
}

// generated code
impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_x = {
			let __white_space_value = value.clone();
			__white_space_value.set_whitespace(WhiteSpaceType::Indent);

			let inner = Number::parse(&mut __white_space_value);
			value.goto(__white_space_value.position())?;
			inner
		}?;
		let __inner_comma = Comma::parse(value)?;
		let __inner_y = Number::parse(value)?;
		Ok(Self { x: __inner_x, comma: __inner_comma, y: __inner_y})
	}

	fn span(&self) -> Span {
		Span::new(self.x.span().start, self.y.span().end)
	}
}

#[derive(Parsable)]
struct TestValue {
	#[value(69, 420)] x: Number,
	comma: Comma,
	y: Number
}

// generated code
impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_x = match Number::parse(value) {
			Ok(__inner_x) if __inner_x == 69 => __inner_x,
			Ok(__inner_x) if __inner_x == 420 => __inner_x,
			Ok(__inner_x) => return Err(ParseError("Value was not one of the expected values.", value.position())),
			Err(error) => return Err(error)
		};
		let __inner_comma = Comma::parse(value)?;
		let __inner_y = Number::parse(value)?;
		Ok(Self { x: __inner_x, comma: __inner_comma, y: __inner_y})
	}

	fn span(&self) -> Span {
		Span::new(self.x.span().start, self.y.span().end)
	}
}

#[derive(Parsable)]
struct TestWhiteSpaceValue {
	#[whitespace(Indent)]
	#[value(69, 420)]
	x: Number,
	comma: Comma,
	y: Number
}

// generated code
impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_x = match {
			let __white_space_value = value.clone();
			__white_space_value.set_whitespace(WhiteSpaceType::Indent);

			let inner = Number::parse(&mut __white_space_value);
			value.goto(__white_space_value.position())?;
			inner
		} {
			Ok(__inner_x) if __inner_x ==  69 => __inner_x,
			Ok(__inner_x) if __inner_x == 420 => __inner_x,
			Ok(__inner_x) => return Err(ParseError("Value was not one of the expected values.", value.position())),
			Err(error) => return Err(error)
		};
		let __inner_comma = Comma::parse(value)?;
		let __inner_y = Number::parse(value)?;
		Ok(Self { x: __inner_x, comma: __inner_comma, y: __inner_y})
	}

	fn span(&self) -> Span {
		Span::new(self.x.span().start, self.y.span().end)
	}
}
```
for enums it looks like this:
```rust
#[derive(Parsable)]
enum Test {
	TestOne(tokens::Hyphen, Number),
	TestTwo {
		x: Number,
		y: Number
	}
}

impl Test {
	fn __parse_testone(value: &mut CharStream) -> Result<Self, ParseError> {
		let __inner_0 = tokens::Hyphen::parse(value)?;
		let __inner_1 = Number::parse(value)?;
		Ok(Self::TestOne(__inner_0, __inner_1))
	}
	fn __parse_testtwo(&mut CharStream) -> Result<Self, ParseError> {
		let __inner_x = Number::parse(value)?;
		let __inner_y = Number::parse(value)?;
		Ok(Self::TestTwo(x: __inner_x, y: __inner_y))
	}
}

impl Parse for Test {
	fn parse(value: &mut CharStream) -> Result<Self, ParseError> {
		let mut options = Vec::new();
		let mut error = None;
		let __value = value.clone();
		match __parse_testone(__value) {
			Ok(inner) => {
				value.goto(__value.position());
				options.push(inner);
			}
			Err(err) => error = Some(err);
		};
		let __value = value.clone();
		match __parse_testtwo(__value) {
			Ok(inner) => {
				value.goto(__value.position());
				options.push(inner);
			}
			Err(err) => error = Some(err);
		};
		if options.len() > 0 {
			Ok(options.sort_by(|a, b| a.span().partial_cmp(b.span()).unwrap())[0])
		} else {
			Err(error.unwrap())
		}
	}

	fn span(&self) -> Span {
		match self {
			Self::TestOne(start, end) => Span::new(start.span().start, end.span().end),
			Self::TestTwo(x, y) => Span::new(x.span().start, y.span().end)
		}
	}
}
```
The helper attributes for enums will work basically the same inside the helper functions as they did for structs. Therefore we do not need to think too much about their implementation right now (I hope...).