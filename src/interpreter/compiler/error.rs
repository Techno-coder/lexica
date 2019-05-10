use std::fmt;

use crate::source::Spanned;

use super::{InterpreterError, ParserError, Token};

pub type CompileResult<'a, T> = Result<T, Spanned<CompileError<'a>>>;

/// An error that can occur during the compilation stage.
#[derive(Debug, Clone)]
pub enum CompileError<'a> {
	UndefinedLabel(String),
	UndefinedFunction(String),
	UnexpectedOperand(Token<'a>),
	Parser(ParserError<'a>),
	Interpreter(InterpreterError),
}

impl<'a> fmt::Display for CompileError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CompileError::*;
		match self {
			UndefinedLabel(identifier) => write!(f, "Label: {}, does not exist", identifier),
			UndefinedFunction(identifier) => write!(f, "Function: {}, does not exist", identifier),
			UnexpectedOperand(operand) => write!(f, "Unexpected instruction operand: {:?}", operand),
			Parser(error) => write!(f, "Parser error occurred while compiling: {}", error),
			Interpreter(error) => write!(f, "Interpreter error occurred while compiling: {}", error),
		}
	}
}
