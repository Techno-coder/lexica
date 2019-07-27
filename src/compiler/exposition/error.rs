use std::fmt;

use crate::node::{Identifier, VariableTarget};

#[derive(Debug)]
pub enum ExpositionError<'a> {
	UndefinedVariable(Identifier<'a>),
	DroppedVariable(VariableTarget<'a>),
}

impl<'a> fmt::Display for ExpositionError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ExpositionError::*;
		match self {
			UndefinedVariable(identifier) => write!(f, "Variable: {}, is not defined", identifier),
			DroppedVariable(target) => write!(f, "Variable: {}, has been dropped", target),
		}
	}
}
