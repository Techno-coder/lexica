use std::fmt;

use crate::source::Spanned;

use super::{Argument, Token};

pub type ParserResult<'a, T> = Result<T, Spanned<ParserError<'a>>>;

#[derive(Debug)]
pub enum ParserError<'a> {
	InvalidAnnotation(&'a str),
	InvalidApplication(&'a str),
	InvalidOperation(&'a str),
	MissingPolarization(&'a str),
	UnexpectedToken(Token<'a>),
	InvalidArgument(Token<'a>),
	UnexpectedArgument(&'a Argument<'a>),
	UnexpectedOperand(Token<'a>),
	InvalidSize(&'a str),
	EndOfInput,
}

impl<'a> fmt::Display for ParserError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use self::ParserError::*;
		match self {
			InvalidAnnotation(identifier) => writeln!(f, "Annotation identifier: {}, is invalid", identifier),
			InvalidApplication(identifier) => writeln!(f, "Invalid application of annotation: {}", identifier),
			InvalidOperation(identifier) => writeln!(f, "Invalid instruction operation: {}", identifier),
			MissingPolarization(identifier) => writeln!(f, "Instruction operation must be polarized: {}", identifier),
			UnexpectedToken(token) => writeln!(f, "Unexpected token: {:?}", token),
			InvalidArgument(argument) => writeln!(f, "Invalid annotation argument token: {:?}", argument),
			UnexpectedArgument(argument) => writeln!(f, "Unexpected annotation argument: {:?}", argument),
			UnexpectedOperand(operand) => writeln!(f, "Unexpected instruction operand: {:?}", operand),
			InvalidSize(size) => writeln!(f, "Specified size is invalid: {}", size),
			EndOfInput => writeln!(f, "Unexpected end of input"),
		}
	}
}
