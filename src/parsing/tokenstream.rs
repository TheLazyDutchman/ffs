use core::fmt;

#[derive(Clone, Default)]
pub struct Position {
    row: usize,
    collumn: usize,
}

impl Position {
    pub fn new(row: usize, collumn: usize) -> Self {
        Self { row, collumn }
    }

    pub fn advance(&mut self) -> &mut Self {
        self.collumn += 1;
        self
    }

    pub fn newline(&mut self) -> &mut Self {
        self.collumn = 0;
        self.row += 1;
        self
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.collumn)
    }
}

#[derive(Default)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {:?}", self.start, self.end)
    }
}

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Number,
    WhiteSpace,
}

pub struct Token {
    value: String,
    span: Span,
    tokentype: TokenType,
}

impl Token {
    pub fn new(value: String, span: Span, tokentype: TokenType) -> Self {
        Self { value, span, tokentype }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({}, from {:?})", self.tokentype, self.value, self.span)
    }
}

pub trait TokenStream: Iterator<Item = Token> {
    fn pos(&self) -> Position;
}
