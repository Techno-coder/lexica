use std::fmt;

use super::{BinaryOperation, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
	Variable(Identifier<'a>),
	LiteralInteger(i64),
	BinaryOperation(Box<BinaryOperation<'a>>),
}

impl<'a> NodeConstruct<'a> for Expression<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression(self)
	}
}

impl<'a> fmt::Display for Expression<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Expression::Variable(identifier) => write!(f, "{}", identifier),
			Expression::LiteralInteger(integer) => write!(f, "{}", integer),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
		}
	}
}
