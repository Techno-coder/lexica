use std::collections::HashMap;
use std::sync::Arc;

use crate::basic::Projection;
use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::*;
use crate::span::{Span, Spanned};

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
	for (index, parameter) in function_type.parameters.iter().enumerate() {
		let Parameter(binding, ascription) = &parameter.node;
		pattern::bind_pattern(&mut environment, engine, binding);

		let binding_type = pattern::binding_type(&mut environment, engine, binding);
		let ascription_type = pattern::template_type(&mut environment, engine, ascription);
		engine.unify(binding_type, ascription_type.clone()).map_err(|error|
			Diagnostic::new(Spanned::new(error, parameter.span)))?;

		let variable = engine.new_variable();
		environment.variable(Variable::new_temporary(index), variable, parameter.span);
		engine.unify(Arc::new(InferenceType::Variable(variable)), ascription_type)
			.map_err(|error| Diagnostic::new(Spanned::new(error, parameter.span)))?;
	}

	let expression = super::expression(context, &function.context,
		&mut environment, engine, &function.expression)?;
	projection(context, &function.context, &mut environment, engine)?;

	let return_type = pattern::template_type(&mut environment,
		engine, &function_type.return_type.node);
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
			Expression::Field(receiver, field) => projection_field(context, function,
				environment, engine, &expression_key, expression_span, receiver, &field)?,
			Expression::MethodCall(receiver, method, arguments) => projection_method(context, function,
				environment, engine, &expression_key, expression_span, receiver, method, arguments)?,
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

fn projection_field(context: &Context, function: &FunctionContext, environment: &mut Environment,
                    engine: &mut TypeEngine, expression: &ExpressionKey, expression_span: Span,
                    receiver: &ExpressionKey, field: &Spanned<Arc<str>>) -> Result<(), Diagnostic> {
	let span = function[receiver].span;
	let inference = engine.find(environment[receiver].clone());
	let inference = dereference(engine, inference);
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
			engine.unify(environment[expression].clone(), field_type)
		}
		InferenceType::Variable(variable) =>
			Err(InferenceError::Unresolved(*variable)),
		InferenceType::Template(_) => {
			let projection = Projection::Field(field.node.clone());
			Err(InferenceError::TemplateProjection(projection))
		}
		InferenceType::Reference(_, _) => unreachable!(),
	}.map_err(|error| Diagnostic::new(Spanned::new(error, expression_span)))
}

fn projection_method(context: &Context, function: &FunctionContext, environment: &mut Environment,
                     engine: &mut TypeEngine, expression: &ExpressionKey, expression_span: Span,
                     receiver: &ExpressionKey, method: &Spanned<Arc<str>>,
                     arguments: &[ExpressionKey]) -> Result<(), Diagnostic> {
	let inference = engine.find(environment[receiver].clone());
	let inference = dereference(engine, inference);
	match &*inference {
		InferenceType::Instance(path, _) => {
			let function_path = method.clone().map(|method|
				Arc::new(FunctionPath::method(path.clone(), method)));
			let function_type = match context.declarations_function.contains_key(&function_path.node) {
				true => crate::node::function_type(context, &function_path),
				false => {
					let error = InferenceError::UndefinedMethod(path.clone(), method.node.clone());
					Err(Diagnostic::new(Spanned::new(error, method.span)))
				}
			}?;

			if arguments.len() + 1 != function_type.parameters.len() {
				let error = InferenceError::FunctionArity(arguments.len() + 1, function_type.parameters.len());
				return Err(Diagnostic::new(Spanned::new(error, method.span)));
			}

			let templates = &mut HashMap::new();
			let Parameter(_, ascription) = &function_type.parameters.first().unwrap().node;
			let ascription = pattern::ascription(environment, engine, templates, ascription);
			let ascription = dereference(engine, ascription);
			engine.unify(inference, ascription).map_err(|error|
				Diagnostic::new(Spanned::new(error, function[receiver].span)))?;

			Iterator::zip(arguments.iter(), function_type.parameters.iter().skip(1))
				.try_for_each(|(argument, parameter)| {
					let Parameter(_, ascription) = &parameter.node;
					let ascription = pattern::ascription(environment, engine, templates, ascription);
					engine.unify(environment[argument].clone(), ascription).map_err(|error|
						Diagnostic::new(Spanned::new(error, function[argument].span)))
				})?;

			let return_type = pattern::ascription(environment,
				engine, templates, &function_type.return_type.node);
			engine.unify(environment[expression].clone(), return_type)
		}
		InferenceType::Variable(variable) =>
			Err(InferenceError::Unresolved(*variable)),
		InferenceType::Template(_) =>
			Err(InferenceError::TemplateMethodCall(method.node.clone())),
		InferenceType::Reference(_, _) => unreachable!(),
	}.map_err(|error| Diagnostic::new(Spanned::new(error, expression_span)))
}

fn dereference(engine: &mut TypeEngine, mut inference: Arc<InferenceType>) -> Arc<InferenceType> {
	while let InferenceType::Reference(_, reference) = &*inference {
		inference = engine.find(reference.clone());
	}
	inference
}
