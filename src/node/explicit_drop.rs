use std::fmt;

use crate::source::Spanned;

use super::{ExpressionNode, NodeConstruct, NodeVisitor, VariableTarget};

#[derive(Debug)]
pub struct ExplicitDrop<'a> {
	pub target: Spanned<VariableTarget<'a>>,
	pub expression: Spanned<ExpressionNode<'a>>,
}

impl<'a> NodeConstruct<'a> for Spanned<ExplicitDrop<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.explicit_drop(self)
	}
}

impl<'a> fmt::Display for ExplicitDrop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "drop {} = {}", self.target, self.expression)
	}
}
