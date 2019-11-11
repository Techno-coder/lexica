use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::*;
use crate::span::Spanned;

use super::{Environment, InferenceError, pattern, TypeContext, TypeEngine, TypeResolution};

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

	let expression = super::expression(context, &function.context,
		&mut environment, engine, &function.expression)?;
	access(context, &function.context, &mut environment, engine)?;

	let return_type = pattern::ascription_type(&mut environment, engine,
		&function_type.return_type.node);
	engine.unify(expression, return_type).map_err(|error|
		Diagnostic::new(Spanned::new(error, function_type.return_type.span)))?;

	let type_context = Arc::new(environment.context(&function.context, engine)?);
	context.type_contexts.insert(function_path.node.clone(), type_context.clone());
	Ok(type_context)
}

/// Resolves field and method call types.
fn access(context: &Context, function: &FunctionContext, environment: &mut Environment,
          engine: &mut TypeEngine) -> Result<(), Diagnostic> {
	for (index, expression) in function.expressions.iter().enumerate() {
		let expression_key = ExpressionKey(index);
		match &expression.node {
			Expression::Field(expression, field) => {
				let span = function[expression].span;
				let type_resolution = engine.construct(environment[expression].clone())
					.map_err(|error| Diagnostic::new(Spanned::new(error, span)))?;
				let TypeResolution(structure_path, _) = type_resolution;
				let structure_path = Spanned::new(Arc::new(structure_path), span);
				let structure = crate::node::structure(context, &structure_path)?;

				let field_type = structure.fields.get(&field.node).ok_or({
					let error = InferenceError::UndefinedField(structure_path.node, field.node.clone());
					Diagnostic::new(Spanned::new(error, field.span))
				}).map(|pattern| super::pattern::ascription_type(environment, engine, pattern))?;
				engine.unify(field_type, environment[&expression_key].clone())
			}
			_ => Ok(()),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, expression.span)))?;
	}
	Ok(())
}
