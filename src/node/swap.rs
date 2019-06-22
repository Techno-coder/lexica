use std::fmt;

use super::{Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub struct Swap<'a> {
	pub left: Identifier<'a>,
	pub right: Identifier<'a>,
}

impl<'a> NodeConstruct<'a> for Swap<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.swap(self)
	}
}

impl<'a> fmt::Display for Swap<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} <=> {}", self.left, self.right)
	}
}
