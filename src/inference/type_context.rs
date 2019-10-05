use std::collections::HashMap;
use std::sync::Arc;

use crate::error::Diagnostic;
use crate::node::{Ascription, ExpressionKey, FunctionContext};
use crate::span::{Span, Spanned};

use super::{DataType, InferenceError, InferenceType, TypeRegister, TypeVariable};

#[derive(Debug, Default)]
pub struct TypeContext {
	type_variables: HashMap<ExpressionKey, Spanned<TypeVariable>>,
	constraints: Vec<(Spanned<TypeVariable>, Spanned<TypeVariable>)>,
	variable_parents: HashMap<TypeVariable, TypeVariable>,
	inference_types: HashMap<TypeVariable, InferenceType>,
	next_variable: usize,
}

impl TypeContext {
	/// Adds the constraint that the types of the two expressions are equal.
	pub fn equate(&mut self, context: &FunctionContext, left: &ExpressionKey, right: &ExpressionKey) {
		let left = self.variable(left, context[left].span);
		let right = self.variable(right, context[right].span);
		self.constraints.push((left, right));
	}

	/// Adds the constraint that the expression type is equal to the ascription.
	pub fn ascribe(&mut self, context: &FunctionContext,
	               expression: &ExpressionKey, ascription: Spanned<Ascription>) {
		let ascription_variable = self.new_variable();
		let expression = self.variable(expression, context[expression].span);
		self.inference_types.insert(ascription_variable, ascription.node.into());
		self.constraints.push((expression, Spanned::new(ascription_variable, ascription.span)));
	}

	pub fn solve(mut self) -> Result<TypeRegister, Diagnostic> {
		std::mem::take(&mut self.constraints).into_iter()
			.try_for_each(|(left, right)| self.unify(left.node, right.node)
				.map_err(|error| Diagnostic::new(Spanned::new(error, right.span))))?;

		let mut expression_types = HashMap::new();
		let mut data_types: HashMap<_, Arc<DataType>> = HashMap::new();
		for (expression_key, variable) in std::mem::take(&mut self.type_variables) {
			let root_type = self.find(variable.node).unwrap();
			let data_type = match data_types.get(&root_type) {
				Some(root_type) => root_type,
				None => data_types.entry(root_type)
					.or_insert(Arc::new(self.construct(root_type).map_err(|error|
						Diagnostic::new(Spanned::new(error, variable.span)))?)),
			}.clone();
			expression_types.insert(expression_key, data_type);
		}

		Ok(TypeRegister::new(expression_types))
	}

	fn construct(&mut self, variable: TypeVariable) -> Result<DataType, InferenceError> {
		let root_type = self.remove(variable).unwrap();
		let data_type = match &root_type {
			InferenceType::Instance(structure, variables) => {
				let variables: Result<Vec<_>, _> = variables.iter().map(|variable|
					self.construct(*variable)).collect();
				DataType(structure.clone(), variables?)
			}
			InferenceType::Variable(variable) => return Err(InferenceError::Unresolved(*variable)),
		};

		self.inference_types.insert(variable, root_type);
		Ok(data_type)
	}

	fn variable(&mut self, expression_key: &ExpressionKey, span: Span) -> Spanned<TypeVariable> {
		match self.type_variables.get(expression_key) {
			Some(type_variable) => type_variable.clone(),
			None => {
				let variable = Spanned::new(self.new_variable(), span);
				self.type_variables.entry(*expression_key).or_insert(variable).clone()
			}
		}
	}

	fn new_variable(&mut self) -> TypeVariable {
		self.next_variable += 1;
		let variable = TypeVariable(self.next_variable - 1);
		*self.variable_parents.entry(variable).or_insert(variable)
	}

	fn unify(&mut self, left: TypeVariable, right: TypeVariable) -> Result<(), InferenceError> {
		let left_type = self.remove(left).ok_or(InferenceError::Recursive(left))?;
		let right_type = self.remove(right).ok_or(InferenceError::Recursive(right))?;
		match (&left_type, &right_type) {
			(InferenceType::Variable(left), _) => self.union(right, *left),
			(_, InferenceType::Variable(right)) => self.union(left, *right),
			(InferenceType::Instance(left, left_variables),
				InferenceType::Instance(right, right_variables)) => {
				match left == right && left_variables.len() == right_variables.len() {
					false => return Err(InferenceError::Unification(left_type, right_type)),
					true => Iterator::zip(left_variables.iter(), right_variables.iter())
						.try_for_each(|(left, right)| self.unify(*left, *right))?,
				}
			}
		}

		self.inference_types.insert(left, left_type);
		self.inference_types.insert(right, right_type);
		Ok(())
	}

	fn remove(&mut self, variable: TypeVariable) -> Option<InferenceType> {
		let root = self.find(variable)?;
		self.inference_types.remove(&root)
	}

	fn find(&mut self, variable: TypeVariable) -> Option<TypeVariable> {
		let parent = *self.variable_parents.get(&variable)?;
		Some(match parent == variable {
			true => variable,
			false => {
				let root = self.find(parent)?;
				self.variable_parents.insert(variable, root);
				root
			}
		})
	}

	fn union(&mut self, representative: TypeVariable, other: TypeVariable) {
		assert_eq!(self.variable_parents.get(&representative), Some(&representative));
		assert_eq!(self.variable_parents.get(&other), Some(&other));
		self.variable_parents.insert(other, representative);
		self.inference_types.remove(&other);
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use crate::declaration::{DeclarationPath, ModulePath, StructurePath};

	use super::*;

	#[test]
	pub fn test_unify() {
		let mut context = TypeContext::default();
		let truth_type: InferenceType = Ascription(truth_structure()).into();
		let (left, right) = (context.new_variable(), context.new_variable());
		context.inference_types.insert(left, truth_type.clone());
		context.inference_types.insert(right, InferenceType::Variable(right));

		assert!(context.unify(left, right).is_ok());
		let root = context.find(right).unwrap();
		let root_type = &context.inference_types[&root];
		assert_eq!(root_type, &truth_type);
	}

	#[test]
	pub fn test_false_unification() {
		let mut context = TypeContext::default();
		let (left, right) = (context.new_variable(), context.new_variable());
		context.inference_types.insert(left, Ascription(truth_structure()).into());
		context.inference_types.insert(right, Ascription(integer_structure()).into());
		assert!(context.unify(left, right).is_err());
	}

	#[test]
	pub fn test_recursive() {
		let mut context = TypeContext::default();
		let (left, right) = (context.new_variable(), context.new_variable());
		let recursive_instance = InferenceType::Instance(truth_structure(), vec![left]);
		context.inference_types.insert(left, recursive_instance);
		let recursive_instance = InferenceType::Instance(truth_structure(), vec![right]);
		context.inference_types.insert(right, recursive_instance);
		assert!(context.unify(left, right).is_err());
	}

	fn truth_structure() -> StructurePath {
		StructurePath(DeclarationPath {
			module_path: ModulePath::intrinsic(),
			identifier: "truth".into(),
		})
	}

	fn integer_structure() -> StructurePath {
		StructurePath(DeclarationPath {
			module_path: ModulePath::intrinsic(),
			identifier: "u32".into(),
		})
	}
}
