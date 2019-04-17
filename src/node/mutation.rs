use std::fmt;

use super::{Expression, Identifier};
use super::NodeConstruct;

#[derive(Debug)]
pub enum Mutation<'a> {
	AddAssign(Identifier<'a>, Expression<'a>),
}

impl<'a> NodeConstruct<'a> for Mutation<'a> {}

impl<'a> fmt::Display for Mutation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Mutation::AddAssign(identifier, expression) => write!(f, "{} += {}", identifier, expression),
		}
	}
}
