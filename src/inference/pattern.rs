use std::collections::HashMap;
use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::{Ascription, AscriptionPattern, BindingPattern, BindingVariable,
	ExpressionPattern, FunctionContext, Pattern, VariablePattern};

use super::{Environment, InferenceType, TypeEngine};

pub fn bind_pattern(environment: &mut Environment, engine: &mut TypeEngine, binding: &BindingPattern) {
	binding.traverse(&mut |terminal| {
		let BindingVariable(variable, _) = &terminal.node;
		environment.variable(variable.clone(), engine.new_variable(), terminal.span);
	});
}

pub fn binding_type(environment: &Environment, engine: &mut TypeEngine,
                    binding: &BindingPattern) -> Arc<InferenceType> {
	match binding {
		Pattern::Wildcard => engine.new_variable_type(),
		Pattern::Terminal(terminal) => {
			let BindingVariable(terminal, _) = &terminal.node;
			Arc::new(InferenceType::Variable(environment[terminal]))
		}
		Pattern::Tuple(patterns) => {
			let binding_types = patterns.iter().map(|pattern|
				binding_type(environment, engine, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), binding_types))
		}
	}
}

pub fn variable_type(environment: &Environment, engine: &mut TypeEngine,
                     variable: &VariablePattern) -> Arc<InferenceType> {
	match variable {
		Pattern::Wildcard => panic!("Wildcard variable is invalid"),
		Pattern::Terminal(terminal) => Arc::new(InferenceType::Variable(environment[&terminal.node])),
		Pattern::Tuple(patterns) => {
			let variable_types = patterns.iter().map(|pattern|
				variable_type(environment, engine, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), variable_types))
		}
	}
}

/// Constructs an inference type from an ascription.
/// Templates are replaced with type variables and are not added to the environment.
pub fn ascription(environment: &mut Environment, engine: &mut TypeEngine,
                  templates: &mut HashMap<Arc<str>, Arc<InferenceType>>,
                  pattern: &AscriptionPattern) -> Arc<InferenceType> {
	match pattern {
		Pattern::Wildcard => engine.new_variable_type(),
		Pattern::Terminal(terminal) => match &terminal.node {
			Ascription::Template(template) => templates.entry(template.clone())
				.or_insert_with(|| engine.new_variable_type()).clone(),
			Ascription::Reference(permission, _, reference) => {
				let ascription = ascription(environment, engine, templates, reference);
				Arc::new(InferenceType::Reference(*permission, ascription))
			}
			Ascription::Structure(structure, structure_templates) => {
				let templates = structure_templates.iter().map(|template|
					ascription(environment, engine, templates, template)).collect();
				Arc::new(InferenceType::Instance(structure.clone(), templates))
			}
		}
		Pattern::Tuple(patterns) => {
			let ascription_types = patterns.iter().map(|pattern|
				ascription(environment, engine, templates, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), ascription_types))
		}
	}
}

/// Constructs an inference type from an ascription.
/// Templates are constructed as template inference types.
pub fn template_type(environment: &mut Environment, engine: &mut TypeEngine,
                     ascription: &AscriptionPattern) -> Arc<InferenceType> {
	match ascription {
		Pattern::Wildcard => engine.new_variable_type(),
		Pattern::Terminal(terminal) => match &terminal.node {
			Ascription::Template(template) =>
				Arc::new(InferenceType::Template(template.clone())),
			Ascription::Reference(permission, _, reference) => {
				let ascription = template_type(environment, engine, reference);
				Arc::new(InferenceType::Reference(*permission, ascription))
			}
			Ascription::Structure(structure, templates) => {
				let templates = templates.iter().map(|template|
					template_type(environment, engine, template)).collect();
				Arc::new(InferenceType::Instance(structure.clone(), templates))
			}
		}
		Pattern::Tuple(patterns) => {
			let ascription_types = patterns.iter().map(|pattern|
				template_type(environment, engine, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), ascription_types))
		}
	}
}

pub fn expression_pattern(context: &Context, function: &FunctionContext, environment: &mut Environment,
                          engine: &mut TypeEngine, pattern: &ExpressionPattern)
                          -> Result<Arc<InferenceType>, Diagnostic> {
	match pattern {
		Pattern::Wildcard => panic!("Wildcard expression is invalid"),
		Pattern::Terminal(terminal) =>
			super::expression(context, function, environment, engine, terminal),
		Pattern::Tuple(patterns) => {
			let expression_types: Result<Vec<_>, _> = patterns.iter().map(|pattern|
				expression_pattern(context, function, environment, engine, pattern)).collect();
			Ok(Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), expression_types?)))
		}
	}
}
