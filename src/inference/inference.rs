use std::fmt;
use std::sync::Arc;

use crate::basic::Projection;
use crate::declaration::{ModulePath, StructurePath};
use crate::error::CompileError;
use crate::intrinsic::Intrinsic;

use super::TypeEngine;

#[derive(Debug, PartialEq)]
pub enum InferenceError {
	Unification(Arc<InferenceType>, Arc<InferenceType>),
	Recursive(InferenceType),
	Unresolved(TypeVariable),
	FunctionArity(usize, usize),
	UndefinedField(Arc<StructurePath>, Arc<str>),
	MissingField(Arc<StructurePath>, Arc<str>),
	ResolvedTemplate(Arc<str>, StructurePath),
	TemplateProjection(Projection),
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
			InferenceError::UndefinedField(structure, field) =>
				write!(f, "Field: {}, is not defined on structure: {}", field, structure),
			InferenceError::MissingField(structure, field) =>
				write!(f, "Structure: {}, is missing field: {}", structure, field),
			InferenceError::ResolvedTemplate(template, structure) =>
				write!(f, "Template: {}, cannot be resolved to a structure: {}", template, structure),
			InferenceError::TemplateProjection(projection) =>
				write!(f, "Projection: {:?}, cannot be performed on a template", projection),
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
			InferenceType::Variable(variable) => write!(f, "{}", variable),
			InferenceType::Instance(structure, variables) => {
				write!(f, "{}", structure)?;
				match variables.split_last() {
					None => Ok(()),
					Some((last, slice)) => {
						write!(f, "<")?;
						slice.iter().try_for_each(|variable| write!(f, "{}, ", variable))?;
						write!(f, "{}>", last)
					}
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeResolution {
	Instance(StructurePath, Vec<TypeResolution>),
	Template,
}

impl TypeResolution {
	pub fn intrinsic(&self) -> Option<Intrinsic> {
		match self {
			TypeResolution::Template => None,
			TypeResolution::Instance(StructurePath(path), parameters) => {
				let is_intrinsic = path.module_path == ModulePath::intrinsic();
				match is_intrinsic && parameters.is_empty() {
					true => Intrinsic::parse(&path.identifier),
					false => None,
				}
			}
		}
	}

	pub fn inference(&self, engine: &mut TypeEngine) -> Arc<InferenceType> {
		match self {
			TypeResolution::Template => engine.new_variable_type(),
			TypeResolution::Instance(structure, resolutions) => {
				let inferences = resolutions.iter().map(|resolution|
					resolution.inference(engine)).collect();
				Arc::new(InferenceType::Instance(structure.clone(), inferences))
			}
		}
	}
}

impl fmt::Display for TypeResolution {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TypeResolution::Template => write!(f, "$"),
			TypeResolution::Instance(structure, resolutions) => {
				write!(f, "{}", structure)?;
				match resolutions.split_last() {
					None => Ok(()),
					Some((last, slice)) => {
						write!(f, "<")?;
						slice.iter().try_for_each(|resolution| write!(f, "{}, ", resolution))?;
						write!(f, "{}>", last)
					}
				}
			}
		}
	}
}

impl Intrinsic {
	pub fn inference(&self) -> Arc<InferenceType> {
		Arc::new(InferenceType::Instance(self.structure(), Vec::new()))
	}
}
