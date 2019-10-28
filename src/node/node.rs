use std::fmt;
use std::sync::Arc;

use crate::declaration::DeclarationPath;
use crate::error::CompileError;

#[derive(Debug)]
pub enum NodeError {
	DroppedVariable(Arc<str>),
	UndefinedVariable(Arc<str>),
	DuplicateParameter(Arc<str>),
	ResolutionConflict(DeclarationPath),
	UnresolvedResolution(DeclarationPath),
}

impl fmt::Display for NodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			NodeError::DroppedVariable(variable) =>
				write!(f, "Variable: {}, is already dropped", variable),
			NodeError::UndefinedVariable(variable) =>
				write!(f, "Variable: {}, is not defined", variable),
			NodeError::DuplicateParameter(parameter) =>
				write!(f, "Parameter: {}, is already defined", parameter),
			NodeError::ResolutionConflict(structure) =>
				write!(f, "Structure: {}, has conflicting resolutions", structure),
			NodeError::UnresolvedResolution(structure) =>
				write!(f, "Structure: {}, has no matching resolutions", structure),
		}
	}
}

impl From<NodeError> for CompileError {
	fn from(error: NodeError) -> Self {
		CompileError::Node(error)
	}
}
