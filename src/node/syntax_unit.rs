use std::collections::HashMap;
use std::fmt;

use crate::source::Spanned;

use super::{Function, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub struct SyntaxUnit<'a> {
	pub functions: HashMap<Identifier<'a>, Spanned<Function<'a>>>,
}

impl<'a> NodeConstruct<'a> for Spanned<SyntaxUnit<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.syntax_unit(self)
	}
}

impl<'a> fmt::Display for SyntaxUnit<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.functions.values().try_for_each(|function| writeln!(f, "{}\n", function))
	}
}
