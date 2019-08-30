use std::fmt;

use crate::source::Spanned;

use super::{ExpressionBlock, ExpressionNode, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub struct WhenConditional<'a> {
	pub branches: Vec<WhenBranch<'a>>,
}

impl<'a> WhenConditional<'a> {
	pub fn unit_evaluation(&self) -> bool {
		self.branches.iter().all(|branch| branch.unit_evaluation())
	}
}

impl<'a> NodeConstruct<'a> for Spanned<WhenConditional<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.when_conditional(self)
	}
}

impl<'a> fmt::Display for WhenConditional<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::utility::IndentWriter;

		write!(f, "when")?;
		match self.branches.len() {
			1 => {
				let branch = &self.branches[0];
				write!(f, " {}", branch.condition)?;
				if let Some(end_condition) = &branch.end_condition {
					write!(f, " => {}", end_condition)?;
				}

				write!(f, " {}", branch.expression_block)
			}
			_ => {
				writeln!(f, " {{")?;
				let mut indent = IndentWriter::wrap(f);
				self.branches.iter().try_for_each(|branch| writeln!(indent, "{}", branch))?;
				write!(f, "}}")
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct WhenBranch<'a> {
	pub condition: Spanned<ExpressionNode<'a>>,
	pub end_condition: Option<Spanned<ExpressionNode<'a>>>,
	pub expression_block: Spanned<ExpressionBlock<'a>>,
}

impl<'a> WhenBranch<'a> {
	pub fn unit_evaluation(&self) -> bool {
		self.expression_block.unit_evaluation()
	}
}

impl<'a> fmt::Display for WhenBranch<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.condition)?;
		if let Some(end_condition) = &self.end_condition {
			write!(f, " => {}", end_condition)?;
		}

		let statements = &self.expression_block.block.statements;
		match statements.len() {
			0 => write!(f, " -> {},", self.expression_block.expression),
			1 if self.unit_evaluation() => write!(f, " -> {},", &statements[0]),
			_ => write!(f, " -> {},", self.expression_block),
		}
	}
}
