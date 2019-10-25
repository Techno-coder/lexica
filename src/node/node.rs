use std::fmt;
use std::sync::Arc;

use crate::error::CompileError;

#[derive(Debug)]
pub enum NodeError {
	ResolutionConflict(Arc<str>),
	UnresolvedResolution(Arc<str>),
}

impl fmt::Display for NodeError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			NodeError::ResolutionConflict(identifier) =>
				write!(f, "Structure: {}, has conflicting resolutions", identifier),
			NodeError::UnresolvedResolution(identifier) =>
				write!(f, "Structure: {}, has no matching resolutions", identifier),
		}
	}
}

impl From<NodeError> for CompileError {
	fn from(error: NodeError) -> Self {
		CompileError::Node(error)
	}
}
