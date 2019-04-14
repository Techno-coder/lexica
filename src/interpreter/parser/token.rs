use crate::source::Span;

#[derive(Debug)]
pub struct Token<'a> {
	pub span: Span,
	pub token_type: TokenType<'a>,
}

#[derive(Debug)]
pub enum TokenType<'a> {
	ReversalHint,
	Annotation(&'a str),
	Label(&'a str),
	LocalLabel(&'a str),
	FunctionLabel(&'a str),
	ReverseLabel(&'a str),
	Equal,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
	UnsignedInteger(u64),
	SignedInteger(i64),
	Float(f64),
	Advance(&'a str),
	Reverse(&'a str),
	Comment(&'a str),
	Identifier(&'a str),
}
