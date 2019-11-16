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

	let return_type = pattern::template_type(&mut environment, engine,
		&function_type.return_type.node);
	engine.unify(expression, return_type).map_err(|error|
		Diagnostic::new(Spanned::new(error, function_type.return_type.span)))?;

	let type_context = Arc::new(environment.context(&function.context, engine)?);
	context.type_contexts.insert(function_path.node.clone(), type_context.clone());
	Ok(type_context)
}

/// Resolves field and method call and dereference types.
fn projection(context: &Context, function: &FunctionContext, environment: &mut Environment,
              engine: &mut TypeEngine) -> Result<(), Diagnostic> {
	for (index, expression) in function.expressions.iter().enumerate() {
		let expression_key = ExpressionKey(index);
		let expression_span = expression.span;
		match &expression.node {
			Expression::Field(expression, field) => {
				let span = function[expression].span;
				let mut inference = engine.find(environment[expression].clone());
				while let InferenceType::Reference(_, reference) = &*inference {
					inference = engine.find(reference.clone());
				}

				match &*inference {
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
						engine.unify(environment[&expression_key].clone(), field_type)
					}
					InferenceType::Variable(variable) =>
						Err(InferenceError::Unresolved(*variable)),
					InferenceType::Template(_) => {
						let projection = Projection::Field(field.node.clone());
						Err(InferenceError::TemplateProjection(projection))
					}
					InferenceType::Reference(_, _) => unreachable!(),
				}.map_err(|error| Diagnostic::new(Spanned::new(error, expression_span)))?;
			}
			Expression::Unary(operator, expression) => match operator.node {
				UnaryOperator::Dereference => {
					let inference = engine.find(environment[expression].clone());
					match &*inference {
						InferenceType::Reference(_, inference) =>
							engine.unify(environment[&expression_key].clone(), inference.clone()),
						_ => Err(InferenceError::Dereference(inference)),
					}.map_err(|error| Diagnostic::new(Spanned::new(error, expression_span)))?;
				}
				_ => (),
			}
			_ => (),
		}
	}
	Ok(())
}
