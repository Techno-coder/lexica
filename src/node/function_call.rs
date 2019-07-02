use std::fmt;

use crate::source::Spanned;

use super::{Expression, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
	pub function: Spanned<Identifier<'a>>,
	pub arguments: Vec<Spanned<Expression<'a>>>,
}

impl<'a> NodeConstruct<'a> for Spanned<&mut FunctionCall<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.function_call(self)
	}
}

impl<'a> fmt::Display for FunctionCall<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}(", self.function)?;

		let split = self.arguments.split_last();
		if let Some((last, rest)) = split {
			rest.iter().try_for_each(|argument| write!(f, "{}, ", argument))?;
			write!(f, "{}", last)?;
		}

		write!(f, ")")
	}
}
