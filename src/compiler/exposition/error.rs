use std::fmt;

use crate::node::Identifier;

#[derive(Debug)]
pub enum ExpositionError<'a> {
	UndefinedVariable(Identifier<'a>)
}

impl<'a> fmt::Display for ExpositionError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ExpositionError::*;
		match self {
			UndefinedVariable(identifier) => write!(f, "Variable: {}, is not defined", identifier),
		}
	}
}
