use std::fmt;

use crate::source::Spanned;

use super::{Binding, ConditionalLoop, ExplicitDrop, Mutation, NodeConstruct, NodeVisitor};

#[derive(Debug)]
pub enum Statement<'a> {
	Binding(Spanned<Binding<'a>>),
	Mutation(Spanned<Mutation<'a>>),
	ExplicitDrop(Spanned<ExplicitDrop<'a>>),
	ConditionalLoop(Spanned<ConditionalLoop<'a>>),
}

impl<'a> NodeConstruct<'a> for Spanned<Statement<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.statement(self)
	}
}

impl<'a> fmt::Display for Statement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Statement::Binding(binding) => write!(f, "{};", binding),
			Statement::Mutation(mutation) => write!(f, "{};", mutation),
			Statement::ExplicitDrop(explicit_drop) => write!(f, "{};", explicit_drop),
			Statement::ConditionalLoop(conditional_loop) => write!(f, "{}", conditional_loop),
		}
	}
}
