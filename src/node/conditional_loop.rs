use std::fmt;

use crate::source::Spanned;

use super::{ExpressionNode, NodeConstruct, NodeVisitor, Statement};

#[derive(Debug)]
pub struct ConditionalLoop<'a> {
	pub start_condition: Option<Spanned<ExpressionNode<'a>>>,
	pub end_condition: Spanned<ExpressionNode<'a>>,
	pub statements: Vec<Spanned<Statement<'a>>>,
}

impl<'a> NodeConstruct<'a> for Spanned<ConditionalLoop<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.conditional_loop(self)
	}
}

impl<'a> fmt::Display for ConditionalLoop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::display::IndentWriter;

		if let Some(start_condition) = &self.start_condition {
			writeln!(f, "loop {} => {} {{", start_condition, self.end_condition)?;
		} else {
			writeln!(f, "loop {} {{", self.end_condition)?;
		}

		let mut indent = IndentWriter::wrap(f);
		self.statements.iter().try_for_each(|statement| writeln!(indent, "{}", statement))?;
		write!(f, "}}")
	}
}
