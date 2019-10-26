use std::fmt;

use crate::declaration::DeclarationPath;
use crate::error::CompileError;

#[derive(Debug)]
pub enum NodeError {
	ResolutionConflict(DeclarationPath),
	UnresolvedResolution(DeclarationPath),
}

impl fmt::Display for NodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
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
