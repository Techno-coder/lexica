use std::sync::Arc;

use crate::basic::Projection;
use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
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
		let ascription_type = pattern::template_type(&mut environment, engine, ascription);
		engine.unify(binding_type, ascription_type).map_err(|error|
			Diagnostic::new(Spanned::new(error, parameter.span)))?;
	}

	let expression = super::expression(context, &function.context,
		&mut environment, engine, &function.expression)?;
	projection(context, &function.context, &mut environment, engine)?;

	let return_type = pattern::ascription_type(&mut environment, engine,
		&function_type.return_type.node);
	engine.unify(expression, return_type).map_err(|error|
		Diagnostic::new(Spanned::new(error, function_type.return_type.span)))?;

	let type_context = Arc::new(environment.context(&function.context, engine)?);
	context.type_contexts.insert(function_path.node.clone(), type_context.clone());
	Ok(type_context)
}

/// Resolves field and method call types.
fn projection(context: &Context, function: &FunctionContext, environment: &mut Environment,
              engine: &mut TypeEngine) -> Result<(), Diagnostic> {
	let templates = environment.templates(engine)?;
	for (index, expression) in function.expressions.iter().enumerate() {
		let expression_key = ExpressionKey(index);
		match &expression.node {
			Expression::Field(expression, field) => {
				let span = function[expression].span;
				match &*engine.find(environment[expression].clone()) {
					InferenceType::Instance(path, inferences) => {
						let path = Spanned::new(Arc::new(path.clone()), span);
						let structure = crate::node::structure(context, &path)?;
						let templates = &mut Iterator::zip(structure.templates.iter(), inferences.iter())
							.map(|(template, inference)| (template.node.clone(), inference.clone()))
							.collect();

						let field_type = structure.fields.get(&field.node).ok_or({
							let error = InferenceError::UndefinedField(path.node, field.node.clone());
							Diagnostic::new(Spanned::new(error, field.span))
						}).map(|pattern| super::pattern::ascription(environment, engine, templates, pattern))?;
						engine.unify(field_type, environment[&expression_key].clone())
							.map_err(|error| Diagnostic::new(Spanned::new(error, span)))?;
					}
					InferenceType::Variable(variable) => if !templates.contains(&variable) {
						let projection = Projection::Field(field.node.clone());
						let error = InferenceError::TemplateProjection(projection);
						return Err(Diagnostic::new(Spanned::new(error, span)));
					}
				}
			}
			_ => (),
		}
	}
	Ok(())
}
