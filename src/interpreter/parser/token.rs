use std::fmt;

#[derive(Debug, Clone)]
pub enum Token<'a> {
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
	AdvanceOnAdvancing(&'a str),
	AdvanceOnReversing(&'a str),
	ReverseOnAdvancing(&'a str),
	ReverseOnReversing(&'a str),
	Comment(&'a str),
	Identifier(&'a str),
}

impl<'a> Token<'a> {
	pub fn element_delimiter(&self) -> bool {
		match self {
			Token::ReversalHint => true,
			Token::Annotation(_) => true,
			Token::Label(_) => true,
			Token::LocalLabel(_) => true,
			Token::FunctionLabel(_) => true,
			Token::ReverseLabel(_) => true,
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
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Token::ReversalHint => write!(f, "*"),
			Token::Annotation(string) => write!(f, "@{}", string),
			Token::Label(string) => write!(f, "{}:", string),
			Token::LocalLabel(string) => write!(f, ".{}:", string),
			Token::FunctionLabel(string) => write!(f, "+{}:", string),
			Token::ReverseLabel(string) => write!(f, "-{}^", string),
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
		}
	}
}
