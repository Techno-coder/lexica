use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::basic::{Item, Reversibility};
use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::{CompileError, Diagnostic};
use crate::node::{ExpressionKey, Variable};
use crate::span::Spanned;

use super::{EvaluationContext, EvaluationFrame};

#[derive(Debug)]
pub enum EvaluationError {
	ArithmeticOverflow,
	UnreachableBranch,
	RuntimeExpression,
}

impl fmt::Display for EvaluationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EvaluationError::ArithmeticOverflow =>
				write!(f, "Arithmetic operation overflow"),
			EvaluationError::UnreachableBranch =>
				write!(f, "Unreachable branch encountered"),
			EvaluationError::RuntimeExpression =>
				write!(f, "Expression is not available at compile time"),
		}
	}
}

impl From<EvaluationError> for CompileError {
	fn from(error: EvaluationError) -> Self {
		CompileError::Evaluation(error)
	}
}

/// Fully evaluates a function and provides the return value.
pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                arguments: Vec<Item>) -> Result<Item, Diagnostic> {
	let function = crate::basic::function(context,
		function_path, Reversibility::Entropic)?;

	let mut frame = EvaluationFrame::new(function);
	arguments.into_iter().enumerate().for_each(|(index, item)|
		frame.context.insert(Variable::new_temporary(index), item));

	let context = &mut EvaluationContext::new(context, Reversibility::Entropic, frame);
	loop { if let Some(item) = context.advance()? { return Ok(item); } }
}

/// Fully evaluates an expression and provides the expression result.
pub fn expression(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                  expression: &ExpressionKey, variables: HashMap<Variable, Item>)
                  -> Result<Item, Diagnostic> {
	let function = crate::basic::expression(context,
		function_path, expression, Reversibility::Entropic)?;

	let mut frame = EvaluationFrame::new(Arc::new(function));
	variables.into_iter().for_each(|(variable, item)|
		frame.context.insert(variable, item));

	let context = &mut EvaluationContext::new(context, Reversibility::Entropic, frame);
	loop { if let Some(item) = context.advance()? { return Ok(item); } }
}
