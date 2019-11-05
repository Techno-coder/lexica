use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::*;
use crate::span::Spanned;

use super::{Environment, InferenceType, pattern, TypeEngine};

pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>) -> Result<(), Diagnostic> {
	let function = crate::node::function(context, function_path)?;
	let function_type = &function.function_type;

	let environment = &mut Environment::new();
	let engine = &mut TypeEngine::default();
	for parameter in &function_type.parameters {
		let Parameter(binding, ascription) = &parameter.node;
		pattern::bind_pattern(environment, engine, binding);

		let binding_type = pattern::binding_type(environment, engine, binding);
		let ascription_type = pattern::ascription_type(environment, engine, ascription);
		engine.unify(binding_type, ascription_type).map_err(|error|
			Diagnostic::new(Spanned::new(error, parameter.span)))?;
	}

	let expression = expression(&function.context, environment, engine, function.expression)?;
	let return_type = pattern::ascription_type(&environment, engine, &function_type.return_type.node);
	engine.unify(expression, return_type).map_err(|error|
		Diagnostic::new(Spanned::new(error, function_type.return_type.span)))
}

pub fn expression(context: &FunctionContext, environment: &mut Environment, engine: &mut TypeEngine,
                  expression_key: ExpressionKey) -> Result<Arc<InferenceType>, Diagnostic> {
	let expression_node = &context[&expression_key];
	let span = expression_node.span;
	Ok(match &expression_node.node {
		Expression::Block(block) => {
			let mut last_expression = None;
			for expression_key in block {
				let expression = expression(context, environment, engine, *expression_key)?;
				last_expression = Some(expression);
			}
			last_expression.unwrap()
		}
		Expression::Binding(binding, ascription, expression_key) => {
			pattern::bind_pattern(environment, engine, &binding.node);
			let binding_type = pattern::binding_type(environment, engine, &binding.node);
			let ascription_type = ascription.as_ref()
				.map(|ascription| pattern::ascription_type(environment, engine, ascription))
				.unwrap_or_else(|| engine.new_variable_type());
			engine.unify(binding_type.clone(), ascription_type)
				.map_err(|error| Diagnostic::new(Spanned::new(error, binding.span)))?;

			let expression = expression(context, environment, engine, *expression_key)?;
			engine.unify(binding_type, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::TerminationLoop(condition_start, condition_end, expression_key) => {
			if let Some(condition_start) = condition_start {
				let condition_type = expression(context, environment, engine, *condition_start)?;
				engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
					Diagnostic::new(Spanned::new(error, context[condition_start].span)))?;
			}

			let condition_type = expression(context, environment, engine, *condition_end)?;
			engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
				Diagnostic::new(Spanned::new(error, context[condition_end].span)))?;
			expression(context, environment, engine, *expression_key)?;
			Intrinsic::Unit.inference()
		}
		Expression::Conditional(branches) => branches.iter()
			.try_fold(engine.new_variable_type(), |inference_type, branch| {
				let (condition_start, condition_end, expression_key) = branch;
				if let Some(condition_end) = condition_end {
					let condition_type = expression(context, environment, engine, *condition_end)?;
					engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
						Diagnostic::new(Spanned::new(error, context[condition_end].span)))?;
				}

				let condition_type = expression(context, environment, engine, *condition_start)?;
				engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
					Diagnostic::new(Spanned::new(error, context[condition_start].span)))?;
				let expression_type = expression(context, environment, engine, *expression_key)?;
				engine.unify(inference_type.clone(), expression_type).map_err(|error|
					Diagnostic::new(Spanned::new(error, span)))?;
				Ok(inference_type)
			})?,
		Expression::Mutation(_, mutable, expression_key) => {
			let mutable = expression(context, environment, engine, *mutable)?;
			let expression = expression(context, environment, engine, *expression_key)?;
			engine.unify(mutable, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::ExplicitDrop(variable, expression_key) => {
			let variable_type = pattern::variable_type(environment, engine, variable);
			let expression = expression(context, environment, engine, *expression_key)?;
			engine.unify(variable_type, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::Unary(_, expression_key) =>
			expression(context, environment, engine, *expression_key)?,
		Expression::Binary(operator, left, right) => {
			let left = expression(context, environment, engine, *left)?;
			let right = expression(context, environment, engine, *right)?;
			engine.unify(left.clone(), right).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			match operator.node {
				BinaryOperator::Arithmetic(_) => left,
				_ => Intrinsic::Truth.inference(),
			}
		}
		Expression::Pattern(pattern) =>
			pattern::expression_pattern(context, environment, engine, pattern)?,
		Expression::Variable(variable) =>
			Arc::new(InferenceType::Variable(environment[variable])),
		Expression::Unsigned(_) => engine.new_variable_type(),
		Expression::Signed(_) => engine.new_variable_type(),
		Expression::Truth(_) => Intrinsic::Truth.inference(),
	})
}
