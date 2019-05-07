use std::fmt;

use crate::source::Spanned;

use super::{Argument, InterpreterError, Token};

pub type ParserResult<'a, T> = Result<T, Spanned<ParserError<'a>>>;

/// An error that can occur during the parsing stage.
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
	InvalidReverseLabelPosition(&'a str),
	UndefinedFunction(String),
	UndefinedLabel(String),
	DuplicateReverseLabel(&'a str),
	DuplicateLabel(&'a str),
	DuplicateLocalLabel(&'a str),
	InvalidSize(&'a str),
	IrreversibleOperation(&'a str),
	IrreversibleCall,
	EndOfInput,
	Interpreter(InterpreterError),
}

impl<'a> fmt::Display for ParserError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ParserError::*;
		match self {
			InvalidAnnotation(identifier) => write!(f, "Annotation identifier: {}, is invalid", identifier),
			InvalidApplication(identifier) => write!(f, "Invalid application of annotation: {}", identifier),
			InvalidOperation(identifier) => write!(f, "Invalid instruction operation: {}", identifier),
			MissingPolarization(identifier) => write!(f, "Instruction operation must be polarized: {}", identifier),
			UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
			InvalidArgument(argument) => write!(f, "Invalid annotation argument token: {:?}", argument),
			UnexpectedArgument(argument) => write!(f, "Unexpected annotation argument: {:?}", argument),
			UnexpectedOperand(operand) => write!(f, "Unexpected instruction operand: {:?}", operand),
			IsolatedReverseLabel(label) => write!(f, "Reverse label: {}, does not have function label", label),
			UndefinedFunction(identifier) => write!(f, "Function label: {}, does not exist", identifier),
			UndefinedLabel(identifier) => write!(f, "Label: {}, does not exist", identifier),
			FunctionMissingContext => write!(f, "Instruction operation must be after function label"),
			LabelMissingContext(local_label) => write!(f, "Local label: {}, is missing a global label", local_label),
			InvalidReverseLabelPosition(label) => write!(f, "Reverse label: {}, is in an invalid position", label),
			DuplicateReverseLabel(label) => write!(f, "Function reverse label: {}, has already been defined", label),
			DuplicateLabel(label) => write!(f, "Label: {}, has already been defined", label),
			DuplicateLocalLabel(label) => write!(f, "Local label: {}, has already been defined", label),
			InvalidSize(size) => write!(f, "Specified size {}, is invalid", size),
			IrreversibleOperation(operation) => write!(f, "Instruction operation: {}, is not reversible", operation),
			IrreversibleCall => write!(f, "Function with no reverse label is not reversible"),
			EndOfInput => write!(f, "Unexpected end of input"),
			Interpreter(error) => write!(f, "Interpreter error occurred while parsing: {}", error),
		}
	}
}
