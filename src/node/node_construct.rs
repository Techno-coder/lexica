use std::fmt::{Debug, Display};
use std::marker::Sized;

use crate::node::*;
use crate::source::Spanned;

pub trait NodeConstruct<'a>: Debug + Display {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result;
}

pub trait NodeVisitor<'a> {
	type Result;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result;
	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result;
	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result;
	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result;
	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result;

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result where Self: Sized {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
			Statement::Expression(expression) => expression.accept(self),
		}
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result;
	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result;
	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result;
	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result;
	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result;
	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result;
	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result;
}
