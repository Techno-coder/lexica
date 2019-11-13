use std::collections::HashMap;
use std::sync::Arc;

use super::{InferenceError, InferenceType, TypeResolution, TypeVariable};

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

	pub fn construct<F>(&mut self, inference_type: Arc<InferenceType>, predicate: &mut F)
	                    -> Result<TypeResolution, InferenceError> where F: FnMut(&TypeVariable) -> bool {
		match inference_type.as_ref() {
			InferenceType::Variable(_) => {
				let inference_type = self.find(inference_type.clone());
				match *inference_type {
					InferenceType::Instance(_, _) => self.construct(inference_type, predicate),
					InferenceType::Variable(variable) => match predicate(&variable) {
						false => Err(InferenceError::Unresolved(variable)),
						true => Ok(TypeResolution::Template)
					}
				}
			}
			InferenceType::Instance(structure, variables) => {
				let variables: Result<_, _> = variables.iter().map(|variable|
					self.construct(variable.clone(), predicate)).collect();
				Ok(TypeResolution::Instance(structure.clone(), variables?))
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

	pub fn find(&mut self, inference_type: Arc<InferenceType>) -> Arc<InferenceType> {
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
	use crate::intrinsic::Intrinsic::*;

	use super::*;

	#[test]
	fn test_unification() {
		let mut engine = TypeEngine::default();
		let variables = vec![engine.new_variable_type(), engine.new_variable_type()];
		let inference_tuple = Arc::new(InferenceType::Instance(Tuple.structure(), variables));

		let variables = vec![engine.new_variable_type(), Truth.inference()];
		assert!(engine.unify(inference_tuple.clone(),
			Arc::new(InferenceType::Instance(Tuple.structure(), variables))).is_ok());
		let variables = vec![Unit.inference(), engine.new_variable_type()];
		assert!(engine.unify(inference_tuple.clone(),
			Arc::new(InferenceType::Instance(Tuple.structure(), variables))).is_ok());

		let variables = vec![TypeResolution(Unit.structure(), Vec::new()),
			TypeResolution(Truth.structure(), Vec::new())];
		assert_eq!(engine.construct(inference_tuple),
			Ok(TypeResolution(Tuple.structure(), variables)));
	}

	#[test]
	fn test_unification_error() {
		let mut engine = TypeEngine::default();
		assert!(engine.unify(Truth.inference(), Unit.inference()).is_err());
	}

	#[test]
	fn test_occurs() {
		let mut engine = TypeEngine::default();
		let variable = engine.new_variable_type();
		let other = Arc::new(InferenceType::Instance(Tuple.structure(), vec![variable.clone()]));
		assert!(engine.unify(variable, other).is_err());
	}
}
