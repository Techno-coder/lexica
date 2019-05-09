use std::fmt;

use crate::source::Spanned;

use super::InterpreterError;

pub type CompileResult<'a, T> = Result<T, Spanned<CompileError>>;

/// An error that can occur during the compilation stage.
#[derive(Debug, Clone)]
pub enum CompileError {
	Interpreter(InterpreterError),
}

impl fmt::Display for CompileError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CompileError::*;
		match self {
			Interpreter(error) => write!(f, "Interpreter error occurred while compiling: {}", error),
		}
	}
}
