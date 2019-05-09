use std::fmt;

use crate::source::Spanned;

use super::{Argument, InterpreterError, Token};

pub type ParserResult<'a, T> = Result<T, Spanned<ParserError<'a>>>;

/// An error that can occur during the parsing stage.
#[derive(Debug, Clone)]
pub enum ParserError<'a> {
	InvalidAnnotation(&'a str),
	InvalidApplication(&'a str),
	MissingPolarization(&'a str),
	UnexpectedToken(Token<'a>),
	InvalidArgument(Token<'a>),
	UnexpectedArgument(Argument<'a>),
	UnexpectedOperand(Token<'a>),
	FunctionMissingContext,
	DuplicateFunctionContext,
	UnmatchedFunctionClose,
	UnmatchedFunctionOpen,
	InvalidReverseLabelPosition(&'a str),
	UndefinedFunction(String),
	UndefinedLabel(String),
	DuplicateFunction(&'a str),
	DuplicateLabel(&'a str),
	InvalidSize(&'a str),
	InvalidOperation(&'a str),
	IrreversibleOperation(&'a str),
	EndOfInput,
	Interpreter(InterpreterError),
}

impl<'a> fmt::Display for ParserError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ParserError::*;
		match self {
			InvalidAnnotation(identifier) => write!(f, "Annotation identifier: {}, is invalid", identifier),
			InvalidApplication(identifier) => write!(f, "Invalid application of annotation: {}", identifier),
			MissingPolarization(identifier) => write!(f, "Instruction operation must be polarized: {}", identifier),
			UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
			InvalidArgument(argument) => write!(f, "Invalid annotation argument token: {:?}", argument),
			UnexpectedArgument(argument) => write!(f, "Unexpected annotation argument: {:?}", argument),
			UnexpectedOperand(operand) => write!(f, "Unexpected instruction operand: {:?}", operand),
			UndefinedFunction(identifier) => write!(f, "Function label: {}, does not exist", identifier),
			UndefinedLabel(identifier) => write!(f, "Label: {}, does not exist", identifier),
			FunctionMissingContext => write!(f, "Entity must be used within a function"),
			DuplicateFunctionContext => write!(f, "Function declared inside of existing function"),
			UnmatchedFunctionClose => writeln!(f, "Function close has no matching declaration"),
			UnmatchedFunctionOpen => writeln!(f, "Function declaration has no matching close"),
			InvalidReverseLabelPosition(label) => write!(f, "Reverse label: {}, is in an invalid position", label),
			DuplicateFunction(identifier) => write!(f, "Function: {}, has already been defined", identifier),
			DuplicateLabel(label) => write!(f, "Label: {}, has already been defined", label),
			InvalidSize(size) => write!(f, "Specified size {}, is invalid", size),
			InvalidOperation(identifier) => write!(f, "Invalid instruction operation: {}", identifier),
			IrreversibleOperation(operation) => write!(f, "Instruction operation: {}, is not reversible", operation),
			EndOfInput => write!(f, "Unexpected end of input"),
			Interpreter(error) => write!(f, "Interpreter error occurred while parsing: {}", error),
		}
	}
}
