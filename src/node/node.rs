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
	RuntimeExpression,
	BindingExpression,
	ArgumentType,
}

impl fmt::Display for NodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			NodeError::DroppedVariable(variable) =>
				write!(f, "Variable: {}, has been dropped", variable),
			NodeError::UndefinedVariable(variable) =>
				write!(f, "Variable: {}, is not defined", variable),
			NodeError::DuplicateParameter(parameter) =>
				write!(f, "Parameter: {}, is already defined", parameter),
			NodeError::ResolutionConflict(item) =>
				write!(f, "Item: {}, has conflicting resolutions", item),
			NodeError::UnresolvedResolution(item) =>
				write!(f, "Item: {}, has no matching resolutions", item),
			NodeError::RuntimeExpression =>
				write!(f, "Expression is not available at compile time"),
			NodeError::BindingExpression =>
				write!(f, "Binding pattern does not match expression"),
			NodeError::ArgumentType =>
				write!(f, "Argument type does not match function parameter"),
		}
	}
}

impl From<NodeError> for CompileError {
	fn from(error: NodeError) -> Self {
		CompileError::Node(error)
	}
}
