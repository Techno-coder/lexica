use std::fmt;

use super::{Expression, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
	Equal,
	Add,
	Minus,
	Multiply,
}

impl BinaryOperator {
	pub fn precedence(&self) -> usize {
		match self {
			BinaryOperator::Equal => 1,
			BinaryOperator::Add => 2,
			BinaryOperator::Minus => 2,
			BinaryOperator::Multiply => 3,
		}
	}
}

impl fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let operator = match self {
			BinaryOperator::Equal => "==",
			BinaryOperator::Add => "+",
			BinaryOperator::Minus => "-",
			BinaryOperator::Multiply => "*",
		};
		write!(f, "{}", operator)
	}
}

#[derive(Debug, Clone, PartialEq)]
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
