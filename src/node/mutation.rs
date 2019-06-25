use std::fmt;

use crate::source::Spanned;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub enum Mutation<'a> {
	Swap(Spanned<Identifier<'a>>, Spanned<Identifier<'a>>),
	AddAssign(Spanned<Identifier<'a>>, Spanned<Expression<'a>>),
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
		}
	}
}
