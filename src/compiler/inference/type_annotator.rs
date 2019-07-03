use std::collections::HashMap;

use crate::node::*;
use crate::source::Spanned;

// TODO: Type annotate all expressions, not just identifiers;
// TODO: May require high level representation lowering
#[derive(Debug)]
pub struct TypeAnnotator<'a> {
	environment: HashMap<Identifier<'a>, DataType<'a>>,
}

impl<'a> TypeAnnotator<'a> {
	pub fn new(environment: HashMap<Identifier<'a>, DataType<'a>>) -> Self {
		Self { environment }
	}
}

impl<'a> NodeVisitor<'a> for TypeAnnotator<'a> {
	type Result = ();

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self);
		operation.right.accept(self);
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let identifier = &binding.variable.identifier;
		if let Some(data_type) = self.environment.get(identifier) {
			binding.variable.data_type = Some(data_type.clone());
		}
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.statements.iter_mut().for_each(|statement| statement.accept(self));
		conditional_loop.start_condition.as_mut().unwrap().accept(self);
		conditional_loop.end_condition.accept(self);
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self);
	}

	fn expression(&mut self, expression: &mut Spanned<Expression<'a>>) -> Self::Result {
		match expression.node {
			Expression::BinaryOperation(_) => expression.binary_operation().accept(self),
			Expression::FunctionCall(_) => expression.function_call().accept(self),
			_ => (),
		}
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.statements.iter_mut().for_each(|statement| statement.accept(self));
	}

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		// TODO: Accept syntax unit to type annotate function call return type
		function_call.arguments.iter_mut().for_each(|expression| expression.accept(self));
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(_, _) => (),
			Mutation::AddAssign(_, expression) => expression.accept(self),
			Mutation::MultiplyAssign(_, expression) => expression.accept(self),
		}
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
		}
	}

	fn syntax_unit(&mut self, _: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		panic!("Types cannot be annotated on a syntax unit")
	}
}
