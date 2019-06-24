use crate::node::*;

pub trait NodeVisitor<'a> {
	type Result;

	fn binary_operation(&mut self, operation: &mut BinaryOperation<'a>) -> Self::Result;
	fn binding(&mut self, binding: &mut Binding<'a>) -> Self::Result;
	fn conditional_loop(&mut self, conditional_loop: &mut ConditionalLoop<'a>) -> Self::Result;
	fn explicit_drop(&mut self, explicit_drop: &mut ExplicitDrop<'a>) -> Self::Result;
	fn expression(&mut self, expression: &mut Expression<'a>) -> Self::Result;
	fn function(&mut self, function: &mut Function<'a>) -> Self::Result;
	fn mutation(&mut self, mutation: &mut Mutation<'a>) -> Self::Result;
	fn statement(&mut self, statement: &mut Statement<'a>) -> Self::Result;
}