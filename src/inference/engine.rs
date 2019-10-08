use std::collections::HashMap;
use std::sync::Arc;

use super::{InferenceError, InferenceType, TypeVariable};

#[derive(Debug, Default)]
pub struct TypeEngine {
	resolutions: HashMap<Arc<InferenceType>, Arc<InferenceType>>,
	next_variable: usize,
}

impl TypeEngine {
	pub fn new_variable_type(&mut self) -> Arc<InferenceType> {
		Arc::new(InferenceType::Variable(self.new_variable()))
	}

	pub fn new_variable(&mut self) -> TypeVariable {
		self.next_variable += 1;
		TypeVariable(self.next_variable - 1)
	}

	pub fn construct(&mut self, inference_type: Arc<InferenceType>) -> Arc<InferenceType> {
		match inference_type.as_ref() {
			InferenceType::Variable(_) => {
				let root = self.find(inference_type);
				self.construct(root)
			}
			InferenceType::Instance(structure, variables) => {
				let variables = variables.iter().map(|variable| self.construct(variable.clone()));
				Arc::new(InferenceType::Instance(structure.clone(), variables.collect()))
			}
		}
	}

	pub fn unify(&mut self, left: Arc<InferenceType>, right: Arc<InferenceType>)
	             -> Result<(), InferenceError> {
		let (left, right) = (self.find(left), self.find(right));
		match (left.as_ref(), right.as_ref()) {
			(InferenceType::Variable(_), InferenceType::Variable(_)) => {
				if left != right {
					self.union(left, right);
				}
			}
			(InferenceType::Variable(variable), representative) => {
				representative.occurs(*variable)?;
				self.union(right, left);
			}
			(representative, InferenceType::Variable(variable)) => {
				representative.occurs(*variable)?;
				self.union(left, right);
			}
			(InferenceType::Instance(left_structure, left_variables),
				InferenceType::Instance(right_structure, right_variables)) => {
				let equivalent_arity = left_variables.len() == right_variables.len();
				match left_structure == right_structure && equivalent_arity {
					false => return Err(InferenceError::Unification(left, right)),
					true => Iterator::zip(left_variables.iter(), right_variables.iter())
						.try_for_each(|(left, right)| self.unify(left.clone(), right.clone()))?,
				}
			}
		}
		Ok(())
	}

	fn find(&mut self, inference_type: Arc<InferenceType>) -> Arc<InferenceType> {
		match self.resolutions.get(&inference_type) {
			None => inference_type,
			Some(parent) => {
				let parent = parent.clone();
				let root = self.find(parent);
				self.resolutions.insert(inference_type, root.clone());
				root
			}
		}
	}

	fn union(&mut self, representative: Arc<InferenceType>, other: Arc<InferenceType>) {
		self.resolutions.insert(other, representative);
	}
}

#[cfg(test)]
mod tests {
	use crate::declaration::DeclarationPath;
	use crate::inference::intrinsic;

	use super::*;

	#[test]
	fn test_unification() {
		let mut engine = TypeEngine::default();
		let variables = vec![engine.new_variable_type(), engine.new_variable_type()];
		let inference_tuple = Arc::new(InferenceType::Instance(intrinsic::tuple(), variables));

		let variables = vec![engine.new_variable_type(), intrinsic::truth()];
		assert!(engine.unify(inference_tuple.clone(),
			Arc::new(InferenceType::Instance(intrinsic::tuple(), variables))).is_ok());
		let variables = vec![intrinsic::unit(), engine.new_variable_type()];
		assert!(engine.unify(inference_tuple.clone(),
			Arc::new(InferenceType::Instance(intrinsic::tuple(), variables))).is_ok());

		let variables = vec![intrinsic::unit(), intrinsic::truth()];
		assert_eq!(engine.construct(inference_tuple).as_ref(),
			&InferenceType::Instance(intrinsic::tuple(), variables));
	}

	#[test]
	fn test_unification_error() {
		let mut engine = TypeEngine::default();
		assert!(engine.unify(intrinsic::truth(), intrinsic::unit()).is_err());
	}

	#[test]
	fn test_occurs() {
		let mut engine = TypeEngine::default();
		let variable = engine.new_variable_type();
		let other = Arc::new(InferenceType::Instance(intrinsic::tuple(), vec![variable.clone()]));
		assert!(engine.unify(variable, other).is_err());
	}
}
