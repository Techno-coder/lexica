use std::fmt;

use crate::source::Spanned;

use super::{Block, ExpressionNode, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub struct ConditionalLoop<'a> {
	pub start_condition: Option<Spanned<ExpressionNode<'a>>>,
	pub end_condition: Spanned<ExpressionNode<'a>>,
	pub block: Spanned<Block<'a>>,
}

impl<'a> NodeConstruct<'a> for Spanned<ConditionalLoop<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.conditional_loop(self)
	}
}

impl<'a> fmt::Display for ConditionalLoop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let end_condition = &self.end_condition;
		match &self.start_condition {
			Some(start_condition) => write!(f, "loop {} => {} ", start_condition, end_condition)?,
			_ => write!(f, "loop {} ", self.end_condition)?,
		}
		write!(f, "{}", self.block)
	}
}
