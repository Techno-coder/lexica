use std::fmt;

use super::{Expression, Identifier};
use super::NodeConstruct;

#[derive(Debug)]
pub struct ExplicitDrop<'a> {
	pub identifier: Identifier<'a>,
	pub expression: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for ExplicitDrop<'a> {}

impl<'a> fmt::Display for ExplicitDrop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "drop {} = {}", self.identifier, self.expression)
	}
}
