use std::fmt;

use crate::source::Spanned;

use super::{Expression, NodeConstruct, NodeVisitor, Variable};

#[derive(Debug)]
pub struct Binding<'a> {
	pub variable: Spanned<Variable<'a>>,
	pub expression: Spanned<Expression<'a>>,
}

impl<'a> NodeConstruct<'a> for Spanned<Binding<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.binding(self)
	}
}

impl<'a> fmt::Display for Binding<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "let {} = {}", self.variable, self.expression)
	}
}
