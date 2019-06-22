use std::fmt;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub struct ExplicitDrop<'a> {
	pub identifier: Identifier<'a>,
	pub expression: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for ExplicitDrop<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.explicit_drop(self)
	}
}

impl<'a> fmt::Display for ExplicitDrop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "drop {} = {}", self.identifier, self.expression)
	}
}
