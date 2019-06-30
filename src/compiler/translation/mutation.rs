use crate::node::Identifier;
use crate::source::{Span, Spanned};

use super::{Element, Evaluation, FunctionContext};

pub fn swap(span: Span, left: &Identifier, right: &Identifier, context: &FunctionContext)
            -> Vec<Spanned<Element>> {
	let (left, right) = (context.get_variable(left), context.get_variable(right));
	vec![instruction!(Advance, format!("swap {} {}", left, right), span)]
}

pub fn mutation_assign(span: Span, identifier: &Identifier, mut expression: Vec<Spanned<Element>>,
                       context: &mut FunctionContext, operation: &str) -> Vec<Spanned<Element>> {
	let mut elements = expression.clone();
	let local_index = context.get_variable(identifier);

	elements.push(instruction!(Advance, match context.pop_evaluation() {
		Evaluation::Local(local) => format!("{} {} {}", operation, local_index, local),
		Evaluation::Immediate(primitive) => format!("{}.i {} {}", operation, local_index, primitive),
	}, span));

	super::polarize_reverse(&mut expression);
	elements.append(&mut expression);
	elements
}

pub fn add_assign(span: Span, identifier: &Identifier, expression: Vec<Spanned<Element>>,
                  context: &mut FunctionContext) -> Vec<Spanned<Element>> {
	mutation_assign(span, identifier, expression, context, "add")
}

pub fn multiply_assign(span: Span, identifier: &Identifier, expression: Vec<Spanned<Element>>,
                       context: &mut FunctionContext) -> Vec<Spanned<Element>> {
	mutation_assign(span, identifier, expression, context, "multiply")
}
