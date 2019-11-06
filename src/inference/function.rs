use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::*;
use crate::span::Spanned;

use super::{Environment, InferenceError, InferenceType, pattern, TypeContext, TypeEngine};

pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                -> Result<Arc<TypeContext>, Diagnostic> {
	if let Some(type_context) = context.type_contexts.get(&function_path.node) {
		return Ok(type_context.clone());
	}

	let function = crate::node::function(context, function_path)?;
	let function_type = &function.function_type;

	let mut environment = Environment::default();
	let engine = &mut TypeEngine::default();
	for parameter in &function_type.parameters {
		let Parameter(binding, ascription) = &parameter.node;
		pattern::bind_pattern(&mut environment, engine, binding);

		let binding_type = pattern::binding_type(&mut environment, engine, binding);
		let ascription_type = pattern::ascription_type(&mut environment, engine, ascription);
		engine.unify(binding_type, ascription_type).map_err(|error|
			Diagnostic::new(Spanned::new(error, parameter.span)))?;
	}

	let expression = expression(context, &function.context, &mut environment, engine, &function.expression)?;
	let return_type = pattern::ascription_type(&environment, engine, &function_type.return_type.node);
	engine.unify(expression, return_type).map_err(|error|
		Diagnostic::new(Spanned::new(error, function_type.return_type.span)))?;

	let type_context = Arc::new(environment.context(&function.context, engine)?);
	context.type_contexts.insert(function_path.node.clone(), type_context.clone());
	Ok(type_context)
}

pub fn expression(context: &Context, function: &FunctionContext, environment: &mut Environment,
                  engine: &mut TypeEngine, expression: &ExpressionKey)
                  -> Result<Arc<InferenceType>, Diagnostic> {
	let inference_type = inference_type(context, function, environment, engine, expression)?;
	environment.expression(*expression, inference_type.clone());
	Ok(inference_type)
}

pub fn inference_type(context: &Context, function: &FunctionContext, environment: &mut Environment,
                      engine: &mut TypeEngine, expression_key: &ExpressionKey)
                      -> Result<Arc<InferenceType>, Diagnostic> {
	let span = function[&expression_key].span;
	Ok(match &function[&expression_key].node {
		Expression::Block(block) => {
			let mut last_expression = None;
			for expression_key in block {
				let expression = expression(context, function, environment, engine, expression_key)?;
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

			let expression = expression(context, function, environment, engine, expression_key)?;
			engine.unify(binding_type, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::TerminationLoop(condition_start, condition_end, expression_key) => {
			if let Some(condition_start) = condition_start {
				let condition_type = expression(context, function, environment, engine, condition_start)?;
				engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
					Diagnostic::new(Spanned::new(error, function[condition_start].span)))?;
			}

			let condition_type = expression(context, function, environment, engine, condition_end)?;
			engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
				Diagnostic::new(Spanned::new(error, function[condition_end].span)))?;
			expression(context, function, environment, engine, expression_key)?;
			Intrinsic::Unit.inference()
		}
		Expression::Conditional(branches) => branches.iter()
			.try_fold(engine.new_variable_type(), |inference_type, branch| {
				let (condition_start, condition_end, expression_key) = branch;
				if let Some(condition_end) = condition_end {
					let condition_type = expression(context, function, environment, engine, condition_end)?;
					engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
						Diagnostic::new(Spanned::new(error, function[condition_end].span)))?;
				}

				let condition_type = expression(context, function, environment, engine, condition_start)?;
				engine.unify(condition_type, Intrinsic::Truth.inference()).map_err(|error|
					Diagnostic::new(Spanned::new(error, function[condition_start].span)))?;
				let expression_type = expression(context, function, environment, engine, expression_key)?;
				engine.unify(inference_type.clone(), expression_type).map_err(|error|
					Diagnostic::new(Spanned::new(error, span)))?;
				Ok(inference_type)
			})?,
		Expression::Mutation(_, mutable, expression_key) => {
			let mutable = expression(context, function, environment, engine, mutable)?;
			let expression = expression(context, function, environment, engine, expression_key)?;
			engine.unify(mutable, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::ExplicitDrop(variable, expression_key) => {
			let variable_type = pattern::variable_type(environment, engine, variable);
			let expression = expression(context, function, environment, engine, expression_key)?;
			engine.unify(variable_type, expression).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			Intrinsic::Unit.inference()
		}
		Expression::FunctionCall(function_path, expressions, _) => {
			let function_type = crate::node::function_type(context,
				&function_path.clone().map(|function_path| Arc::new(function_path)))?;
			if expressions.len() != function_type.parameters.len() {
				let function_arity = function_type.parameters.len();
				let error = InferenceError::FunctionArity(expressions.len(), function_arity);
				return Err(Diagnostic::new(Spanned::new(error, span)));
			}

			Iterator::zip(expressions.iter(), function_type.parameters.iter())
				.try_for_each(|(expression_key, parameter)| {
					let Parameter(_, ascription) = &parameter.node;
					let ascription = pattern::ascription_type(environment, engine, ascription);
					let expression_type = expression(context, function, environment, engine, expression_key)?;
					engine.unify(expression_type, ascription).map_err(|error|
						Diagnostic::new(Spanned::new(error, function[expression_key].span)))
				})?;
			pattern::ascription_type(environment, engine, &function_type.return_type.node)
		}
		Expression::Unary(_, expression_key) =>
			expression(context, function, environment, engine, expression_key)?,
		Expression::Binary(operator, left, right) => {
			let left = expression(context, function, environment, engine, left)?;
			let right = expression(context, function, environment, engine, right)?;
			engine.unify(left.clone(), right).map_err(|error|
				Diagnostic::new(Spanned::new(error, span)))?;
			match operator.node {
				BinaryOperator::Arithmetic(_) => left,
				_ => Intrinsic::Truth.inference(),
			}
		}
		Expression::Pattern(pattern) =>
			pattern::expression_pattern(context, function, environment, engine, pattern)?,
		Expression::Variable(variable) =>
			Arc::new(InferenceType::Variable(environment[variable])),
		Expression::Integer(_) => engine.new_variable_type(),
		Expression::Truth(_) => Intrinsic::Truth.inference(),
		Expression::Item(_) => engine.new_variable_type(),
	})
}
