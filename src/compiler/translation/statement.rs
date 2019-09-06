use crate::basic::*;
use crate::node::VariableTarget;
use crate::source::{Span, Spanned};

use super::Translator;

type Element = Spanned<super::Element>;

impl<'a, 'b> Translator<'a, 'b> {
	pub fn statement(&mut self, statement: &Statement<'a>, elements: &mut Vec<Element>) {
		match statement {
			Statement::Binding(binding) => self.binding(binding, elements),
			Statement::Mutation(mutation) => self.mutation(mutation, elements),
			Statement::Assignment(assignment) => self.assignment(assignment, elements),
			Statement::FunctionCall(function_call) => self.function_call(function_call, elements),
			Statement::ImplicitDrop(implicit_drop) => self.implicit_drop(implicit_drop, elements),
		}
	}

	pub fn mutation(&mut self, mutation: &Spanned<Mutation<'a>>, elements: &mut Vec<Element>) {
		let (target, expression, operation) = match &mutation.node {
			Mutation::Swap(left, right) => {
				let left = self.binding_local(&left.node.clone().into());
				let right = self.binding_local(&right.node.clone().into());
				let instruction = format!("swap {} {}", left, right);
				return elements.push(instruction!(Advance, instruction, mutation.span));
			}
			Mutation::AddAssign(target, expression) => (target, expression, "add"),
			Mutation::MinusAssign(target, expression) => (target, expression, "minus"),
			Mutation::MultiplyAssign(target, expression) => (target, expression, "multiply"),
		};

		let local = self.binding_local(&target.node.clone().into());
		self.mutate(local, expression, operation, elements);
	}

	pub fn mutate(&mut self, local: usize, expression: &Spanned<Expression<'a>>,
	              operation: &str, elements: &mut Vec<Element>) {
		elements.push(instruction!(Advance, match &expression.node {
			Expression::Unit => return,
			Expression::Variable(variable) => {
				let other = self.binding_local(&variable.target.clone().into());
				format!("{} {} {}", operation, local, other)
			},
			Expression::Primitive(primitive) =>
				format!("{}.i {} {}", operation, local, primitive),
		}, expression.span));
	}

	pub fn function_call(&mut self, function_call: &Spanned<FunctionCall<'a>>, elements: &mut Vec<Element>) {
		function_call.arguments.iter().for_each(|argument| self.drop_expression(argument, elements));
		let instruction = format!("call {}", function_call.function.node);
		let span = function_call.function.span;

		match self.is_intrinsic(&function_call.function.node) {
			false => elements.push(instruction!(Advance, instruction, span)),
			true => {
				elements.push(instruction!(Advance, Advance, instruction, span));
				// TODO: Assuming arguments are by reference
				let mut restore_elements = Vec::new();
				function_call.arguments.iter()
					.for_each(|argument| self.drop_expression(argument, &mut restore_elements));
				elements.append(&mut self.invert_elements(restore_elements));
			}
		}
	}

	pub fn drop_expression(&self, expression: &Spanned<Expression<'a>>, elements: &mut Vec<Element>) {
		let span = expression.span;
		match &expression.node {
			Expression::Unit => (),
			Expression::Variable(variable) => self.drop_target(&variable.target, span, elements),
			Expression::Primitive(primitive) => {
				let instruction = format!("drop.i {} {}", primitive.size(), primitive);
				elements.push(instruction!(Advance, instruction, span));
			}
		}
	}

	pub fn drop_target(&self, target: &VariableTarget<'a>, span: Span, elements: &mut Vec<Element>) {
		let incorporates = self.structure_incorporates(target);
		super::structure_primitives(target, &mut |target, _| {
			let instruction = format!("drop {}", self.binding_local(&target));
			elements.push(instruction!(Advance, instruction, span));
		}, true, incorporates);
	}

	pub fn assignment(&mut self, assignment: &Spanned<Assignment<'a>>, elements: &mut Vec<Element>) {
		let local = self.binding_local(&assignment.target.node.clone().into());
		self.assign_expression(local, &assignment.expression, elements);
	}

	pub fn assign_expression(&mut self, local: usize, expression: &Spanned<Expression<'a>>,
	                         elements: &mut Vec<Element>) {
		let span = expression.span;
		match &expression.node {
			Expression::Unit => (),
			Expression::Variable(variable) => {
				let other = self.binding_local(&variable.target.clone().into());
				let instruction = format!("clone {} {}", local, other);
				let instruction = instruction!(Advance, Advance, instruction, span);
				elements.push(instruction);
			}
			Expression::Primitive(primitive) => {
				let instruction = format!("reset {} {}", local, primitive);
				let instruction = instruction!(Advance, Advance, instruction, span);
				elements.push(instruction);
			}
		}
	}

	pub fn implicit_drop(&mut self, implicit_drop: &Spanned<ImplicitDrop<'a>>, elements: &mut Vec<Element>) {
		self.drop_target(&implicit_drop.target, implicit_drop.span, elements);
	}
}
