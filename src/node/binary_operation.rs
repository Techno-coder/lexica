use std::fmt;

use super::{Expression, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub enum BinaryOperator {
	Equal,
	Plus,
	Minus,
}

impl fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let operator = match self {
			BinaryOperator::Equal => "==",
			BinaryOperator::Plus => "+",
			BinaryOperator::Minus => "-",
		};
		write!(f, "{}", operator)
	}
}

#[derive(Debug, Clone)]
pub struct BinaryOperation<'a> {
	pub left: Expression<'a>,
	pub right: Expression<'a>,
	pub operator: BinaryOperator,
}

impl<'a> NodeConstruct<'a> for BinaryOperation<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.binary_operation(self)
	}
}

impl<'a> fmt::Display for BinaryOperation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} {} {}", self.left, self.operator, self.right)
	}
}
