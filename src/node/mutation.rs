use std::fmt;

use crate::source::Spanned;

use super::{ExpressionNode, NodeConstruct, NodeVisitor, VariableTarget};

#[derive(Debug, Clone)]
pub enum Mutation<'a> {
	Swap(Spanned<VariableTarget<'a>>, Spanned<VariableTarget<'a>>),
	AddAssign(Spanned<VariableTarget<'a>>, Spanned<ExpressionNode<'a>>),
	MinusAssign(Spanned<VariableTarget<'a>>, Spanned<ExpressionNode<'a>>),
	MultiplyAssign(Spanned<VariableTarget<'a>>, Spanned<ExpressionNode<'a>>),
}

impl<'a> NodeConstruct<'a> for Spanned<Mutation<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.mutation(self)
	}
}

impl<'a> fmt::Display for Mutation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Mutation::Swap(left, right) => write!(f, "{} <=> {}", left, right),
			Mutation::AddAssign(identifier, expression) => write!(f, "{} += {}", identifier, expression),
			Mutation::MinusAssign(identifier, expression) => write!(f, "{} -= {}", identifier, expression),
			Mutation::MultiplyAssign(identifier, expression) => write!(f, "{} *= {}", identifier, expression),
		}
	}
}
