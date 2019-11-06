use std::fmt;
use std::sync::Arc;

use crate::declaration::{ModulePath, StructurePath};
use crate::error::CompileError;
use crate::intrinsic::Intrinsic;

#[derive(Debug, PartialEq)]
pub enum InferenceError {
	Unification(Arc<InferenceType>, Arc<InferenceType>),
	Recursive(InferenceType),
	Unresolved(TypeVariable),
	FunctionArity(usize, usize),
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
			InferenceError::FunctionArity(expression, function) =>
				write!(f, "Expression arity: {}, is not equal to function: {}", expression, function),
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

#[derive(Debug, PartialEq)]
pub struct TypeResolution(pub StructurePath, pub Vec<TypeResolution>);

impl TypeResolution {
	pub fn intrinsic(&self) -> Option<Intrinsic> {
		let TypeResolution(StructurePath(declaration_path), parameters) = self;
		let is_intrinsic = declaration_path.module_path == ModulePath::intrinsic();
		match is_intrinsic && parameters.is_empty() {
			true => Intrinsic::parse(&declaration_path.identifier),
			false => None,
		}
	}
}

impl Intrinsic {
	pub fn inference(&self) -> Arc<InferenceType> {
		Arc::new(InferenceType::Instance(self.structure(), Vec::new()))
	}
}
