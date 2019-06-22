use std::fmt;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor, Statement, Variable};

#[derive(Debug)]
pub struct Function<'a> {
	pub identifier: Identifier<'a>,
	pub parameters: Vec<Variable<'a>>,
	pub statements: Vec<Statement<'a>>,
	pub return_value: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for Function<'a> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.function(self)
	}
}

impl<'a> fmt::Display for Function<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
