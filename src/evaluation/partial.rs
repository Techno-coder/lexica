use std::collections::HashMap;
use std::sync::Arc;

use chashmap::CHashMap;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{Execution, Expression, ExpressionKey, FunctionContext, NodeFunction};
use crate::span::Spanned;

use super::EvaluationError;

pub type PartialFunctions = CHashMap<Arc<FunctionPath>, Arc<NodeFunction>>;

/// Partially evaluates a function.
pub fn partial_function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                        -> Result<Arc<NodeFunction>, Diagnostic> {
	if let Some(function) = context.partial_functions.get(&function_path.node) {
		return Ok(function.clone());
	}

	let mut function = crate::node::function(context, function_path)?.as_ref().clone();
	crate::inference::function(context, function_path)?;
	for index in 0..function.context.expressions.len() {
		let expression_key = &ExpressionKey(index);
		let expression = &function.context[expression_key];
		let span = expression.span;

		if let Expression::FunctionCall(_, _, Execution::Compile) = &expression.node {
			verify_scope(&mut function.context, expression_key)?;
			let item = super::expression(context, function_path,
				expression_key, HashMap::new()).map_err(|diagnostic|
				diagnostic.note(format!("Invoked from: {}", span.location(context))))?;
			function.context.apply(expression_key, |_, expression|
				expression.node = Expression::Item(item));
		}
	}

	let function = Arc::new(function);
	context.partial_functions.insert(function_path.node.clone(), function.clone());
	Ok(function)
}

/// Checks all expressions are available at compile execution.
fn verify_scope(function: &mut FunctionContext, expression: &ExpressionKey) -> Result<(), Diagnostic> {
	function.traverse(expression, &mut |_, expression| {
		let span = expression.span;
		match expression.node {
			Expression::Variable(_) => {
				let error = Spanned::new(EvaluationError::RuntimeExpression, span);
				Err(Diagnostic::new(error))
			}
			_ => Ok(false)
		}
	})
}
