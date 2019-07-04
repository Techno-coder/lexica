use std::fmt;

use crate::source::Spanned;

use super::{DataType, ExpressionNode, Identifier, NodeConstruct, NodeVisitor, Statement, Variable};

#[derive(Debug)]
pub struct Function<'a> {
	pub identifier: Spanned<Identifier<'a>>,
	pub parameters: Vec<Spanned<Variable<'a>>>,
	pub statements: Vec<Spanned<Statement<'a>>>,
	pub return_value: Spanned<ExpressionNode<'a>>,
	pub return_type: Spanned<DataType<'a>>,
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

		write!(f, "fn {}(", self.identifier)?;
		let split = self.parameters.split_last();
		if let Some((last, rest)) = split {
			rest.iter().try_for_each(|parameter| write!(f, "{}, ", parameter))?;
			write!(f, "{}", last)?;
		}
		writeln!(f, ") -> {} {{", self.return_type.resolved().unwrap())?;

		let mut indent = IndentWriter::wrap(f);
		self.statements.iter().try_for_each(|statement| writeln!(indent, "{}", statement))?;
		writeln!(indent, "{}", self.return_value)?;
		write!(f, "}}")
	}
}
