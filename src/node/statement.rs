use std::fmt;

use crate::source::Spanned;

use super::{Binding, ConditionalLoop, ExplicitDrop, ExpressionNode, Mutation, NodeConstruct,
            NodeVisitor};

#[derive(Debug, Clone)]
pub enum Statement<'a> {
	Binding(Spanned<Binding<'a>>),
	Mutation(Spanned<Mutation<'a>>),
	ExplicitDrop(Spanned<ExplicitDrop<'a>>),
	ConditionalLoop(Spanned<ConditionalLoop<'a>>),
	Expression(Spanned<ExpressionNode<'a>>),
}

impl<'a> Statement<'a> {
	pub fn terminated(&self) -> bool {
		use super::Expression;
		match self {
			Statement::Binding(_) => true,
			Statement::Mutation(_) => true,
			Statement::ExplicitDrop(_) => true,
			Statement::ConditionalLoop(_) => false,
			Statement::Expression(expression) => match expression.expression {
				Expression::WhenConditional(_) => false,
				_ => true,
			}
		}
	}
}

impl<'a> NodeConstruct<'a> for Spanned<Statement<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.statement(self)
	}
}

impl<'a> fmt::Display for Statement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Statement::Binding(binding) => write!(f, "{}", binding),
			Statement::Mutation(mutation) => write!(f, "{}", mutation),
			Statement::ExplicitDrop(explicit_drop) => write!(f, "{}", explicit_drop),
			Statement::ConditionalLoop(conditional_loop) => write!(f, "{}", conditional_loop),
			Statement::Expression(expression) => write!(f, "{}", expression),
		}
	}
}
