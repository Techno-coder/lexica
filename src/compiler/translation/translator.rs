use std::ops::DerefMut;

use crate::interpreter::Size;
use crate::node::*;
use crate::source::Spanned;

use super::{Element, FunctionContext};

#[derive(Debug, Default)]
pub struct Translator<'a> {
	context: FunctionContext<'a>,
}

impl<'a> NodeVisitor<'a> for Translator<'a> {
	type Result = Vec<Spanned<Element>>;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		let mut elements = operation.left.accept(self);
		elements.append(&mut operation.right.accept(self));
		elements.append(&mut super::binary_operation(operation, &mut self.context));
		elements
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let elements = binding.expression.accept(self);
		let local_index = self.context.pop_expression();
		let identifier = binding.variable.identifier.clone();
		let identifier = Spanned::new(identifier, binding.variable.span);
		self.context.annotate_local(local_index, identifier);
		elements
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		let (start_label, end_label) = self.context.pair_labels();
		let mut elements = super::loop_header(conditional_loop.span, start_label, end_label);
		let end_condition = &mut conditional_loop.end_condition;
		let condition = end_condition.accept(self);
		elements.append(&mut super::loop_end_condition(condition, &mut self.context, end_condition, end_label));

		conditional_loop.statements.iter_mut()
			.for_each(|statement| elements.append(&mut statement.accept(self)));

		let start_condition = conditional_loop.start_condition.as_mut().unwrap();
		let condition = start_condition.accept(self);
		elements.append(&mut super::loop_start_condition(condition, &mut self.context, start_condition, start_label));
		elements.append(&mut super::loop_suffix(conditional_loop.span, start_label, end_label));
		elements
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		let mut elements = explicit_drop.expression.accept(self);
		super::polarize_reverse(&mut elements);

		let local_index = self.context.drop_variable(&explicit_drop.identifier);
		let expression_index = self.context.pop_expression();
		let instruction = format!("clone {} {}", local_index, expression_index);
		elements.insert(0, instruction!(Advance, Reverse, instruction, explicit_drop.span));
		elements
	}

	fn expression(&mut self, expression: &mut Spanned<Expression<'a>>) -> Self::Result {
		match &mut expression.node {
			Expression::Variable(variable) => {
				self.context.push_expression(self.context.get_variable(variable));
				Vec::new()
			}
			Expression::LiteralInteger(integer) => {
				let local_index = self.context.register_local(Size::Signed64);
				let instruction = format!("reset {} {}", local_index, integer);
				self.context.push_expression(local_index);
				vec![instruction!(Advance, Advance, instruction, expression.span)]
			}
			Expression::BinaryOperation(operation) => {
				let operation = Box::deref_mut(operation);
				Spanned::new(operation, expression.span).accept(self)
			}
		}
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		super::function_parameters(function, &mut self.context);
		let mut function_elements: Vec<_> = function.statements.iter_mut()
			.flat_map(|statement| statement.accept(self)).collect();
		function_elements.append(&mut function.return_value.accept(self));

		let mut elements = super::function_locals(function.span, &self.context);
		elements.append(&mut super::function_header(function));
		elements.append(&mut super::function_arguments(function));
		elements.append(&mut function_elements);

		let return_index = self.context.pop_expression();
		elements.append(&mut super::function_drops(&self.context, return_index));
		elements.append(&mut super::function_return(function, return_index));
		elements
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		let span = mutation.span;
		match &mut mutation.node {
			Mutation::Swap(left, right) => super::swap(span, left, right, &self.context),
			Mutation::AddAssign(identifier, expression) => {
				let expression = expression.accept(self);
				super::add_assign(span, identifier, expression, &mut self.context)
			}
			Mutation::MultiplyAssign(identifier, expression) => {
				let expression = expression.accept(self);
				super::multiply_assign(span, identifier, expression, &mut self.context)
			}
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
		syntax_unit.functions.iter_mut()
			.flat_map(|(_, function)| {
				let elements = function.accept(self);
				self.context = FunctionContext::default();
				elements
			}).collect()
	}
}
