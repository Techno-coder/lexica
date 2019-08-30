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
		use crate::utility::IndentWriter;
		if !f.alternate() { writeln!(f, "{{")?; }

		{
			let is_alternate = f.alternate();
			let mut indent = IndentWriter::wrap(f);
			let f: &mut dyn fmt::Write = match is_alternate {
				false => &mut indent,
				true => f,
			};

			for statement in &self.statements {
				match statement.terminated() {
					false => writeln!(f, "{}", statement),
					true => writeln!(f, "{};", statement),
				}?;
			}
		}

		if !f.alternate() { write!(f, "}}")?; }
		Ok(())
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
		self.expression.node.evaluation_type == DataType::UNIT
	}
}

impl<'a> NodeConstruct<'a> for Spanned<ExpressionBlock<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression_block(self)
	}
}

impl<'a> fmt::Display for ExpressionBlock<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use super::Expression;
		use crate::utility::IndentWriter;

		writeln!(f, "{{")?;
		let mut indent = IndentWriter::wrap(f);
		write!(indent, "{:#}", self.block)?;

		match self.expression.node.as_ref() {
			Expression::Unit => (),
			_ => writeln!(indent, "{:#}", self.expression)?,
		}

		write!(f, "}}")
	}
}

