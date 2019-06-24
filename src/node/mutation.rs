use std::fmt;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub enum Mutation<'a> {
	Swap(Identifier<'a>, Identifier<'a>),
	AddAssign(Identifier<'a>, Expression<'a>),
}

impl<'a> NodeConstruct<'a> for Mutation<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.mutation(self)
	}
}

impl<'a> fmt::Display for Mutation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Mutation::Swap(left, right) => write!(f, "{} <=> {}", left, right),
			Mutation::AddAssign(identifier, expression) => write!(f, "{} += {}", identifier, expression),
		}
	}
}
