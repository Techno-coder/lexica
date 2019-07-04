use std::fmt;

use polytype::{Type, UnificationError};

use crate::node::Identifier;

#[derive(Debug)]
pub enum TypeError<'a> {
	TypeConflict(Type<Identifier<'a>>, Type<Identifier<'a>>),
}

impl<'a> From<UnificationError<Identifier<'a>>> for TypeError<'a> {
	fn from(error: UnificationError<Identifier<'a>>) -> Self {
		match error {
			UnificationError::Occurs(variable) => panic!("Invalid type: {}, constrained", variable),
			UnificationError::Failure(left, right) => TypeError::TypeConflict(left, right),
		}
	}
}

impl<'a> fmt::Display for TypeError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::TypeError::*;
		match self {
			TypeConflict(left, right) => write!(f, "Type: {}, conflicts with: {}", left, right),
		}
	}
}