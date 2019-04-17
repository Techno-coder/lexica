use std::fmt;

use crate::source::Spanned;

use super::{Argument, InterpreterError, Token};

pub type ParserResult<'a, T> = Result<T, Spanned<ParserError<'a>>>;

#[derive(Debug, Clone)]
pub enum ParserError<'a> {
	InvalidAnnotation(&'a str),
	InvalidApplication(&'a str),
	InvalidOperation(&'a str),
	MissingPolarization(&'a str),
	UnexpectedToken(Token<'a>),
	InvalidArgument(Token<'a>),
	UnexpectedArgument(Argument<'a>),
	UnexpectedOperand(Token<'a>),
	IsolatedReverseLabel(&'a str),
	FunctionMissingContext,
	LabelMissingContext(&'a str),
	UndefinedFunction(String),
	UndefinedLabel(String),
	DuplicateReverseLabel(&'a str),
	DuplicateLabel(&'a str),
	DuplicateLocalLabel(&'a str),
	InvalidSize(&'a str),
	EndOfInput,
	Interpreter(InterpreterError),
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
			IsolatedReverseLabel(label) => writeln!(f, "Reverse label: {}, does not have function label", label),
			UndefinedFunction(identifier) => writeln!(f, "Function label: {}, does not exist", identifier),
			UndefinedLabel(identifier) => writeln!(f, "Label: {}, does not exist", identifier),
			FunctionMissingContext => writeln!(f, "Instruction operation must be after function label"),
			LabelMissingContext(local_label) => writeln!(f, "Local label: {} is missing a global label", local_label),
			DuplicateReverseLabel(label) => writeln!(f, "Function reverse label: {}, has already been defined", label),
			DuplicateLabel(label) => writeln!(f, "Label: {}, has already been defined", label),
			DuplicateLocalLabel(label) => writeln!(f, "Local label: {}, has already been defined", label),
			InvalidSize(size) => writeln!(f, "Specified size {}, is invalid", size),
			EndOfInput => writeln!(f, "Unexpected end of input"),
			Interpreter(error) => writeln!(f, "Interpreter error occurred while parsing: {}", error),
		}
	}
}
