use std::fmt;

use crate::source::Spanned;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor, Statement, Variable};

#[derive(Debug)]
pub struct Function<'a> {
	pub identifier: Spanned<Identifier<'a>>,
	pub parameters: Vec<Spanned<Variable<'a>>>,
	pub statements: Vec<Spanned<Statement<'a>>>,
	pub return_value: Spanned<Expression<'a>>,
}

impl<'a> NodeConstruct<'a> for Spanned<Function<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.function(self)
	}
}

impl<'a> fmt::Display for Function<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::display::IndentWriter;

		write!(f, "(")?;
		let split = self.parameters.split_last();
		if let Some((last, rest)) = split {
			for parameter in rest {
				write!(f, "{}, ", parameter)?;
			}
			write!(f, "{}", last)?;
		}
		writeln!(f, ") {{")?;

		let mut indent = IndentWriter::wrap(f);
		for statement in &self.statements {
			writeln!(indent, "{}", statement)?;
		}
		writeln!(indent, "{}", self.return_value)?;
		write!(f, "}}")
	}
}
