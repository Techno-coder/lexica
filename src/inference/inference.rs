use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::declaration::StructurePath;
use crate::error::CompileError;
use crate::intrinsic::Intrinsic;
use crate::node::Variable;

pub type Environment = HashMap<Variable, TypeVariable>;

#[derive(Debug)]
pub enum InferenceError {
	Unification(Arc<InferenceType>, Arc<InferenceType>),
	Recursive(InferenceType),
	Unresolved(TypeVariable),
}

impl fmt::Display for InferenceError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InferenceError::Unification(left, right) =>
				write!(f, "Types: {}, and: {}, do not match", left, right),
			InferenceError::Recursive(variable) =>
				write!(f, "Inference type: {}, is recursively defined", variable),
			InferenceError::Unresolved(variable) =>
				write!(f, "Inference type: {}, has not been resolved", variable),
		}
	}
}

impl From<InferenceError> for CompileError {
	fn from(error: InferenceError) -> Self {
		CompileError::Inference(error)
	}
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct TypeVariable(pub usize);

impl fmt::Display for TypeVariable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let TypeVariable(variable) = self;
		write!(f, "${}", variable)
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum InferenceType {
	Instance(StructurePath, Vec<Arc<InferenceType>>),
	Variable(TypeVariable),
}

impl InferenceType {
	pub fn occurs(&self, variable: TypeVariable) -> Result<(), InferenceError> {
		match self {
			InferenceType::Instance(_, variables) => variables.iter()
				.try_for_each(|type_variable| type_variable.occurs(variable)),
			InferenceType::Variable(type_variable) => match type_variable == &variable {
				true => Err(InferenceError::Recursive(self.clone())),
				false => Ok(())
			},
		}
	}
}

impl fmt::Display for InferenceType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InferenceType::Instance(structure, variables) => {
				write!(f, "{}", structure)?;
				if let Some((last, rest)) = variables.split_last() {
					write!(f, "<")?;
					rest.iter().try_for_each(|variable| write!(f, "{}, ", variable))?;
					write!(f, "{}>", last)?;
				}
				Ok(())
			}
			InferenceType::Variable(variable) => write!(f, "{}", variable),
		}
	}
}

impl Intrinsic {
	pub fn inference(&self) -> Arc<InferenceType> {
		Arc::new(InferenceType::Instance(self.structure(), Vec::new()))
	}
}
