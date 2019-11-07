use std::fmt;
use std::sync::Arc;

use crate::basic::{Item, Reversibility};
use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::{CompileError, Diagnostic};
use crate::node::Variable;
use crate::span::Spanned;

use super::{EvaluationContext, EvaluationFrame};

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

pub fn evaluate(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                arguments: Vec<Item>) -> Result<Item, Diagnostic> {
	let function = crate::basic::basic_function(context,
		function_path, Reversibility::Entropic)?;

	let mut frame = EvaluationFrame::new(function);
	arguments.into_iter().enumerate().for_each(|(index, item)|
		frame.context.insert(Variable::new_temporary(index), item));

	let context = &mut EvaluationContext::new(context, Reversibility::Entropic, frame);
	loop {
		if let Some(item) = context.advance()? {
			return Ok(item);
		}
	}
}
