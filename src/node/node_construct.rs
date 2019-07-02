use std::fmt::{Debug, Display};

use crate::node::*;
use crate::source::Spanned;

pub trait NodeConstruct<'a>: Debug + Display {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result;
}

pub trait NodeVisitor<'a> {
	type Result;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result;
	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result;
	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result;
	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result;
	fn expression(&mut self, expression: &mut Spanned<Expression<'a>>) -> Self::Result;
	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result;
	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result;
	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result;
	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result;
	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result;
}
