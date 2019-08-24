use crate::basic::{Binding, Value};
use crate::node::BinaryOperator;
use crate::source::Spanned;

use super::Translator;

type Element = Spanned<super::Element>;

impl<'a, 'b> Translator<'a, 'b> {
	pub fn register_binding(&mut self, binding: &Spanned<Binding<'a>>) {
		match &binding.value {
			Value::Expression(expression) if expression.is_unit() => return,
			_ => self.register_variable(&binding.variable),
		};
	}

	pub fn binding(&mut self, binding: &Spanned<Binding<'a>>, elements: &mut Vec<Element>) {
		let span = binding.span;
		match &binding.value {
			Value::Uninitialized(_) => return,
			Value::Expression(expression) => match expression.is_unit() {
				false => {
					let local = self.binding_local(&binding.variable.target);
					self.assign_expression(local, expression, elements)
				}
				true => return,
			},
			Value::FunctionCall(function_call) => {
				let local = self.binding_local(&binding.variable.target);
				self.function_call(function_call, elements);
				let instruction = format!("restore {}", local);
				elements.push(instruction!(Advance, instruction, function_call.span));
			}
			Value::BinaryOperation(binary_operation) => {
				let local = self.binding_local(&binding.variable.target);
				let operation = match binary_operation.operator.node {
					BinaryOperator::Equal => {
						let left = self.promote(&binary_operation.left, elements);
						let right = self.promote(&binary_operation.right, elements);
						let instruction = format!("compare = {} {} {}", left, right, local);
						elements.push(instruction!(Advance, Advance, instruction, span));
						return;
					}
					BinaryOperator::Add => "add",
					BinaryOperator::Minus => "minus",
					BinaryOperator::Multiply => "multiply",
				};

				self.assign_expression(local, &binary_operation.left, elements);
				self.mutate(local, &binary_operation.right, operation, elements);
			}
		}
	}
}
