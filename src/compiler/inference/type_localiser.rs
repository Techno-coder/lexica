use std::collections::HashMap;

use crate::node::*;
use crate::source::Spanned;

/// Clones data types from a global context to local nodes.
#[derive(Debug, Default)]
pub struct TypeLocaliser<'a> {
	function_returns: HashMap<Identifier<'a>, DataType<'a>>,
	function_parameters: HashMap<Identifier<'a>, Vec<DataType<'a>>>,
}

impl<'a> NodeVisitor<'a> for TypeLocaliser<'a> {
	type Result = ();

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self);
		operation.right.accept(self);
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self);
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.statements.iter_mut().for_each(|statement| statement.accept(self));
		conditional_loop.start_condition.as_mut().unwrap().accept(self);
		conditional_loop.end_condition.accept(self);
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self);
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		match &mut expression.expression {
			Expression::BinaryOperation(_) => expression.binary_operation().accept(self),
			Expression::FunctionCall(_) => expression.function_call().accept(self),
			_ => (),
		}
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.statements.iter_mut().for_each(|statement| statement.accept(self));
		function.return_value.accept(self);
	}

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().for_each(|argument| argument.accept(self));
		function_call.evaluation_type = self.function_returns[&function_call.function].clone();

		let parameters = self.function_parameters[&function_call.function].iter();
		for (argument, parameter) in function_call.arguments.iter_mut().zip(parameters) {
			argument.evaluation_type = parameter.clone();
		}
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::AddAssign(_, expression) |
			Mutation::MultiplyAssign(_, expression) => expression.accept(self),
			_ => (),
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

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		for (identifier, function) in &syntax_unit.functions {
			let return_type = function.return_type.node.clone();
			self.function_returns.insert(identifier.clone(), return_type);

			let parameters = function.parameters.iter().map(|parameter| &parameter.data_type);
			self.function_parameters.insert(identifier.clone(), parameters.cloned().collect());
		}

		syntax_unit.functions.values_mut().for_each(|function| function.accept(self));
	}
}
