use std::fmt;

use crate::source::Spanned;

use super::{ExpressionNode, NodeConstruct, NodeVisitor, Statement};

#[derive(Debug, Clone)]
pub struct Block<'a> {
	pub statements: Vec<Spanned<Statement<'a>>>,
}

impl<'a> NodeConstruct<'a> for Spanned<Block<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.block(self)
	}
}

impl<'a> fmt::Display for Block<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Ok(for statement in &self.statements {
			match statement.terminated() {
				false => writeln!(f, "{}", statement),
				true => writeln!(f, "{};", statement),
			}?;
		})
	}
}

#[derive(Debug, Clone)]
pub struct ExpressionBlock<'a> {
	pub block: Spanned<Block<'a>>,
	pub expression: Spanned<ExpressionNode<'a>>,
}

impl<'a> ExpressionBlock<'a> {
	pub fn unit_evaluation(&self) -> bool {
		use super::DataType;
		self.expression.node.evaluation_type == DataType::UNIT_TYPE
	}
}

impl<'a> NodeConstruct<'a> for Spanned<ExpressionBlock<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression_block(self)
	}
}

impl<'a> fmt::Display for ExpressionBlock<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use super::Expression;
		write!(f, "{}", self.block)?;
		match self.expression.expression {
			Expression::Unit => Ok(()),
			_ => writeln!(f, "{}", self.expression),
		}
	}
}

