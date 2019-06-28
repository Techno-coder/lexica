use crate::interpreter::Size;
use crate::node::{BinaryOperation, BinaryOperator};
use crate::source::Spanned;

use super::{Element, FunctionContext};

pub fn binary_operation(operation: &mut Spanned<&mut BinaryOperation>, context: &mut FunctionContext)
                        -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let left_index = context.pop_expression();
	let right_index = context.pop_expression();

	if let BinaryOperator::Equal = *operation.operator {
		let local_index = context.register_local(Size::Boolean);
		context.push_expression(local_index);

		let instruction = format!("compare = {} {} {}", left_index, right_index, local_index);
		elements.push(instruction!(Advance, instruction, operation.span));
		return elements;
	}

	let local_index = context.clone_local(left_index);
	context.push_expression(local_index);

	let instruction = format!("clone {} {}", local_index, left_index);
	elements.push(instruction!(Advance, Advance, instruction, operation.span));

	match *operation.operator {
		BinaryOperator::Add => {
			let instruction = format!("add {} {}", local_index, right_index);
			elements.push(instruction!(Advance, instruction, operation.operator.span));
		}
		BinaryOperator::Minus => {
			let instruction = format!("minus {} {}", local_index, right_index);
			elements.push(instruction!(Advance, instruction, operation.operator.span));
		}
		BinaryOperator::Multiply => {
			let instruction = format!("multiply {} {}", local_index, right_index);
			elements.push(instruction!(Advance, instruction, operation.operator.span));
		}
		_ => (),
	};
	elements
}
