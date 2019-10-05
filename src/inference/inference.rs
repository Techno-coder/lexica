use std::fmt;

use crate::declaration::StructurePath;
use crate::error::CompileError;
use crate::node::Ascription;

#[derive(Debug)]
pub enum InferenceError {
	Unification(InferenceType, InferenceType),
	Recursive(TypeVariable),
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

#[derive(Debug, Clone, PartialEq)]
pub enum InferenceType {
	Instance(StructurePath, Vec<TypeVariable>),
	Variable(TypeVariable),
}

impl From<Ascription> for InferenceType {
	fn from(ascription: Ascription) -> Self {
		let Ascription(ascription) = ascription;
		InferenceType::Instance(ascription, Vec::new())
	}
}

impl fmt::Display for InferenceType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InferenceType::Instance(data_type, variables) => {
				write!(f, "{}", data_type)?;
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
