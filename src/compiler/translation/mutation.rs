use crate::node::VariableTarget;
use crate::source::{Span, Spanned};

use super::{Element, Evaluation, FunctionContext};

pub fn swap(span: Span, left: &VariableTarget, right: &VariableTarget, context: &FunctionContext)
            -> Vec<Spanned<Element>> {
	let (left, right) = (context.get_variable(left), context.get_variable(right));
	vec![instruction!(Advance, format!("swap {} {}", left, right), span)]
}

pub fn mutation_assign(span: Span, target: &VariableTarget, mut expression: Vec<Spanned<Element>>,
                       context: &mut FunctionContext, operation: &str) -> Vec<Spanned<Element>> {
	let mut elements = expression.clone();
	let local_index = context.get_variable(target);

	elements.push(instruction!(Advance, match context.pop_evaluation() {
		Evaluation::Unit => panic!("Unit evaluation cannot be assigned"),
		Evaluation::Local(local) => format!("{} {} {}", operation, local_index, local),
		Evaluation::Immediate(primitive) => format!("{}.i {} {}", operation, local_index, primitive),
	}, span));

	super::polarize_reverse(&mut expression);
	elements.append(&mut expression);
	elements
}

pub fn add_assign(span: Span, target: &VariableTarget, expression: Vec<Spanned<Element>>,
                  context: &mut FunctionContext) -> Vec<Spanned<Element>> {
	mutation_assign(span, target, expression, context, "add")
}

pub fn minus_assign(span: Span, target: &VariableTarget, expression: Vec<Spanned<Element>>,
                    context: &mut FunctionContext) -> Vec<Spanned<Element>> {
	mutation_assign(span, target, expression, context, "minus")
}

pub fn multiply_assign(span: Span, target: &VariableTarget, expression: Vec<Spanned<Element>>,
                       context: &mut FunctionContext) -> Vec<Spanned<Element>> {
	mutation_assign(span, target, expression, context, "multiply")
}
