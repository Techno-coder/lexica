use crate::interpreter::Size;
use crate::node::{BinaryOperation, BinaryOperator};
use crate::source::{Span, Spanned};

use super::{Element, Evaluation, FunctionContext};

pub fn binary_operation(operation: &mut Spanned<BinaryOperation>, context: &mut FunctionContext)
                        -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let left_index = context.pop_evaluation()
		.promote(&mut elements, context);
	let right_value = context.pop_evaluation();

	if let BinaryOperator::Equal = *operation.operator {
		return binary_equality(elements, context, left_index, right_value, operation.span);
	}

	let local_index = context.clone_local(left_index);
	context.push_evaluation(Evaluation::Local(local_index));
	let instruction = format!("clone {} {}", local_index, left_index);
	elements.push(instruction!(Advance, Advance, instruction, operation.span));

	let span = operation.operator.span;
	binary_operator(&mut elements, match *operation.operator {
		BinaryOperator::Add => "add",
		BinaryOperator::Minus => "minus",
		BinaryOperator::Multiply => "multiply",
		_ => panic!("Binary operator is not of homogeneous type"),
	}, local_index, right_value, span);
	elements
}

pub fn binary_equality(mut elements: Vec<Spanned<Element>>, context: &mut FunctionContext,
                       left_index: usize, right_value: Evaluation, span: Span) -> Vec<Spanned<Element>> {
	let local_index = context.register_local(Size::Boolean);
	context.push_evaluation(Evaluation::Local(local_index));

	let right_index = right_value.promote(&mut elements, context);
	let instruction = format!("compare = {} {} {}", left_index, right_index, local_index);
	elements.push(instruction!(Advance, instruction, span));
	return elements;
}

pub fn binary_operator(elements: &mut Vec<Spanned<Element>>, instruction: &str,
                       local_index: usize, right_value: Evaluation, span: Span) {
	elements.push(instruction!(Advance, match right_value {
		Evaluation::Unit => panic!("Unit evaluation cannot be part of an operation"),
		Evaluation::Local(local) => format!("{} {} {}", instruction, local_index, local),
		Evaluation::Immediate(primitive) => format!("{}.i {} {}", instruction, local_index, primitive),
	}, span));
}
