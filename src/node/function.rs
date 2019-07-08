use std::fmt;

use crate::source::Spanned;

use super::{DataType, ExpressionBlock, Identifier, NodeConstruct, NodeVisitor, Variable};

#[derive(Debug)]
pub struct Function<'a> {
	pub identifier: Spanned<Identifier<'a>>,
	pub parameters: Vec<Spanned<Variable<'a>>>,
	pub expression_block: Spanned<ExpressionBlock<'a>>,
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

		match self.return_type.node == DataType::UNIT_TYPE {
			false => writeln!(f, ") -> {} {{", self.return_type.resolved().unwrap()),
			true => writeln!(f, ") {{"),
		}?;

		let mut indent = IndentWriter::wrap(f);
		write!(indent, "{}", self.expression_block)?;
		write!(f, "}}")
	}
}
