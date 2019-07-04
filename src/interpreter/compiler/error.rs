use std::fmt;

use crate::source::Spanned;

use super::parser::{ParserError, Token};
use super::InterpreterError;

pub type CompileResult<'a, T> = Result<T, Spanned<CompileError<'a>>>;

/// An error that can occur during the compilation stage.
#[derive(Debug, Clone)]
pub enum CompileError<'a> {
	MissingPolarization(&'a str),
	UndefinedLabel(String),
	UndefinedFunction(String),
	UnexpectedOperand(Token<'a>),
	IrreversibleOperation(&'a str),
	Parser(ParserError<'a>),
	Interpreter(InterpreterError),
}

impl<'a> fmt::Display for CompileError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CompileError::*;
		match self {
			MissingPolarization(identifier) => write!(f, "Instruction operation must be polarized: {}", identifier),
			UndefinedLabel(identifier) => write!(f, "Label: {}, does not exist", identifier),
			UndefinedFunction(identifier) => write!(f, "Function: {}, does not exist", identifier),
			UnexpectedOperand(operand) => write!(f, "Unexpected instruction operand: {:?}", operand),
			IrreversibleOperation(operation) => write!(f, "Instruction operation: {}, is not reversible", operation),
			Parser(error) => write!(f, "Parser error occurred while compiling: {}", error),
			Interpreter(error) => write!(f, "Interpreter error occurred while compiling: {}", error),
		}
	}
}
