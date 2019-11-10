use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::{Ascription, AscriptionPattern, BindingPattern, BindingVariable,
	ExpressionPattern, FunctionContext, Pattern, VariablePattern};

use super::{Environment, InferenceType, TypeEngine};

pub fn bind_pattern(environment: &mut Environment, engine: &mut TypeEngine, binding: &BindingPattern) {
	match binding {
		Pattern::Wildcard => (),
		Pattern::Terminal(terminal) => {
			let BindingVariable(variable, _) = &terminal.node;
			environment.variable(variable.clone(), engine.new_variable(), terminal.span);
		}
		Pattern::Tuple(patterns) => patterns.iter()
			.for_each(|pattern| bind_pattern(environment, engine, pattern)),
	}
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
		Pattern::Wildcard => engine.new_variable_type(),
		Pattern::Terminal(terminal) => Arc::new(InferenceType::Variable(environment[&terminal.node])),
		Pattern::Tuple(patterns) => {
			let variable_types = patterns.iter().map(|pattern|
				variable_type(environment, engine, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), variable_types))
		}
	}
}

pub fn ascription_type(environment: &Environment, engine: &mut TypeEngine,
                       ascription: &AscriptionPattern) -> Arc<InferenceType> {
	match ascription {
		Pattern::Wildcard => engine.new_variable_type(),
		Pattern::Terminal(terminal) => {
			let Ascription(structure) = &terminal.node;
			Arc::new(InferenceType::Instance(structure.clone(), Vec::new()))
		}
		Pattern::Tuple(patterns) => {
			let ascription_types = patterns.iter().map(|pattern|
				ascription_type(environment, engine, pattern)).collect();
			Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), ascription_types))
		}
	}
}

pub fn expression_pattern(context: &Context, function: &FunctionContext, environment: &mut Environment,
                          engine: &mut TypeEngine, pattern: &ExpressionPattern)
                          -> Result<Arc<InferenceType>, Diagnostic> {
	match pattern {
		Pattern::Wildcard => Ok(engine.new_variable_type()),
		Pattern::Terminal(terminal) =>
			super::expression(context, function, environment, engine, terminal),
		Pattern::Tuple(patterns) => {
			let expression_types: Result<Vec<_>, _> = patterns.iter().map(|pattern|
				expression_pattern(context, function, environment, engine, pattern)).collect();
			Ok(Arc::new(InferenceType::Instance(Intrinsic::Tuple.structure(), expression_types?)))
		}
	}
}
