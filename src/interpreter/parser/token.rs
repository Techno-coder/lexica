use std::fmt;

/// Represents a whitespace separated substring of textual code.
#[derive(Debug, Clone)]
pub enum Token<'a> {
	ReversalHint,
	Annotation(&'a str),
	Label(&'a str),
	Function(&'a str),
	FunctionOpen,
	FunctionClose,
	Equal,
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
	UnsignedInteger(u64),
	SignedInteger(i64),
	Float(f64),
	Reversed(&'a str),
	AdvanceOnAdvancing(&'a str),
	AdvanceOnReversing(&'a str),
	ReverseOnAdvancing(&'a str),
	ReverseOnReversing(&'a str),
	Comment(&'a str),
	Identifier(&'a str),
}

impl<'a> Token<'a> {
	/// Returns whether the token can delineate an element.
	pub fn element_delimiter(&self) -> bool {
		match self {
			Token::ReversalHint => true,
			Token::Annotation(_) => true,
			Token::Label(_) => true,
			Token::Function(_) => true,
			Token::FunctionClose => true,
			Token::Reversed(_) => true,
			Token::AdvanceOnAdvancing(_) => true,
			Token::AdvanceOnReversing(_) => true,
			Token::ReverseOnAdvancing(_) => true,
			Token::ReverseOnReversing(_) => true,
			Token::Comment(_) => true,
			_ => false,
		}
	}
}

impl<'a> fmt::Display for Token<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::ReversalHint => write!(f, "*"),
			Token::Annotation(string) => write!(f, "@{}", string),
			Token::Label(string) => write!(f, "{}:", string),
			Token::Function(string) => write!(f, "~{}", string),
			Token::FunctionOpen => write!(f, "{{"),
			Token::FunctionClose => write!(f, "}}"),
			Token::Equal => write!(f, "="),
			Token::LessThan => write!(f, "<"),
			Token::LessThanEqual => write!(f, "<="),
			Token::GreaterThan => write!(f, ">"),
			Token::GreaterThanEqual => write!(f, ">="),
			Token::UnsignedInteger(integer) => write!(f, "{}", integer),
			Token::SignedInteger(integer) => write!(f, "{}", integer),
			Token::Float(float) => write!(f, "{}", float),
			Token::AdvanceOnAdvancing(string) => write!(f, "+{}", string),
			Token::AdvanceOnReversing(string) => write!(f, "-{}", string),
			Token::ReverseOnAdvancing(string) => write!(f, "+{}'", string),
			Token::ReverseOnReversing(string) => write!(f, "-{}'", string),
			Token::Comment(comment) => write!(f, "{}", comment),
			Token::Identifier(string) => write!(f, "{}", string),
			Token::Reversed(string) => write!(f, "{}'", string),
		}
	}
}
