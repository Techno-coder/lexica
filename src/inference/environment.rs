use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{ExpressionKey, FunctionContext, Variable};
use crate::span::{Span, Spanned};

use super::{InferenceError, InferenceType, TypeEngine, TypeResolution, TypeVariable};

pub type TypeContexts = CHashMap<Arc<FunctionPath>, Arc<TypeContext>>;

#[derive(Debug, Default)]
pub struct Environment {
	templates: HashMap<Arc<str>, (TypeVariable, Span)>,
	variables: HashMap<Variable, (TypeVariable, Span)>,
	expressions: HashMap<ExpressionKey, Arc<InferenceType>>,
}

impl Environment {
	pub fn template(&mut self, engine: &mut TypeEngine, template: &Arc<str>, span: Span) -> Arc<InferenceType> {
		Arc::new(InferenceType::Variable(match self.templates.get(template) {
			Some((type_variable, _)) => type_variable.clone(),
			None => {
				let type_variable = engine.new_variable();
				self.templates.insert(template.clone(), (type_variable, span));
				type_variable
			}
		}))
	}

	pub fn variable(&mut self, variable: Variable, type_variable: TypeVariable, span: Span) {
		if self.variables.insert(variable.clone(), (type_variable, span)).is_some() {
			panic!("Variable: {}, already exists in inference environment", variable);
		}
	}

	pub fn expression(&mut self, expression: ExpressionKey, inference_type: Arc<InferenceType>) {
		if self.expressions.insert(expression, inference_type).is_some() {
			panic!("Expression: {:?}, already exists in inference environment", expression);
		}
	}

	pub fn context(self, function: &FunctionContext, engine: &mut TypeEngine)
	               -> Result<TypeContext, Diagnostic> {
		let templates = self.templates.into_iter().map(|(template, (type_variable, span))| {
			match engine.find(Arc::new(InferenceType::Variable(type_variable))).as_ref() {
				InferenceType::Variable(type_variable) => Ok(*type_variable),
				InferenceType::Instance(structure, _) => {
					let error = InferenceError::ResolvedTemplate(template, Arc::new(structure.clone()));
					Err(Diagnostic::new(Spanned::new(error, span)))
				}
			}
		}).collect::<Result<HashSet<_>, _>>()?;
		let construct = &mut |engine: &mut TypeEngine, inference: Arc<InferenceType>, span| {
			match *engine.find(inference.clone()) {
				InferenceType::Variable(variable) if templates.contains(&variable) => None,
				_ => Some(engine.construct(inference).map_err(|error|
					Diagnostic::new(Spanned::new(error, span))))
			}
		};

		let variables = self.variables.into_iter().filter_map(|(variable, (type_variable, span))|
			construct(engine, Arc::new(InferenceType::Variable(type_variable)), span)
				.map(|resolution| resolution.map(|resolution| (variable, resolution))))
			.collect::<Result<_, _>>()?;
		let expressions = self.expressions.into_iter().filter_map(|(expression, inference)|
			construct(engine, inference, function[&expression].span)
				.map(|resolution| resolution.map(|resolution| (expression, resolution))))
			.collect::<Result<_, _>>()?;
		Ok(TypeContext { variables, expressions })
	}
}

impl Index<&Variable> for Environment {
	type Output = TypeVariable;

	fn index(&self, index: &Variable) -> &Self::Output {
		let (type_variable, _) = &self.variables[index];
		type_variable
	}
}

impl IndexMut<&Variable> for Environment {
	fn index_mut(&mut self, index: &Variable) -> &mut Self::Output {
		let (type_variable, _) = self.variables.get_mut(index).unwrap();
		type_variable
	}
}

impl Index<&ExpressionKey> for Environment {
	type Output = Arc<InferenceType>;

	fn index(&self, index: &ExpressionKey) -> &Self::Output {
		&self.expressions[index]
	}
}

impl IndexMut<&ExpressionKey> for Environment {
	fn index_mut(&mut self, index: &ExpressionKey) -> &mut Self::Output {
		self.expressions.get_mut(index).unwrap()
	}
}

#[derive(Debug)]
pub struct TypeContext {
	variables: HashMap<Variable, TypeResolution>,
	expressions: HashMap<ExpressionKey, TypeResolution>,
}

impl Index<&Variable> for TypeContext {
	type Output = TypeResolution;

	fn index(&self, index: &Variable) -> &Self::Output {
		&self.variables[index]
	}
}

impl Index<&ExpressionKey> for TypeContext {
	type Output = TypeResolution;

	fn index(&self, index: &ExpressionKey) -> &Self::Output {
		&self.expressions[index]
	}
}
