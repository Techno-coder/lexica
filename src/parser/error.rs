use std::fmt;

use crate::parser::Token;
use crate::source::Spanned;

pub type ParserResult<'a, T> = Result<T, Spanned<ParserError<'a>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError<'a> {
	ExpectedToken(Token<'a>),
	ExpectedIdentifier,
	ExpectedExpression,
	ExpectedOperator,
	ExpectedMutator,
	ExpectedStatement,
	UnexpectedToken(Token<'a>),
}

impl<'a> fmt::Display for ParserError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParserError::ExpectedToken(token) => write!(f, "Expected token: {:?}", token),
			ParserError::ExpectedIdentifier => write!(f, "Expected identifier"),
			ParserError::ExpectedExpression => write!(f, "Expected expression"),
			ParserError::ExpectedOperator => write!(f, "Expected operator"),
			ParserError::ExpectedMutator => write!(f, "Expected mutation operator"),
			ParserError::ExpectedStatement => write!(f, "Expected statement"),
			ParserError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
		}
	}
}
