use std::fmt;

use crate::error::CompileError;

#[derive(Debug)]
pub enum EvaluationError {
	ArithmeticOverflow,
	UnreachableBranch,
}

impl fmt::Display for EvaluationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EvaluationError::ArithmeticOverflow =>
				write!(f, "Arithmetic operation overflow"),
			EvaluationError::UnreachableBranch =>
				write!(f, "Unreachable branch encountered"),
		}
	}
}

impl From<EvaluationError> for CompileError {
	fn from(error: EvaluationError) -> Self {
		CompileError::Evaluation(error)
	}
}
