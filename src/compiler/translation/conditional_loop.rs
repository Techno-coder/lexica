use crate::interpreter::Direction;
use crate::node::Expression;
use crate::source::{Span, Spanned};

use super::{Element, FunctionContext};

pub fn loop_header(loop_span: Span, start_label: usize, end_label: usize) -> Vec<Spanned<Element>> {
	let mut elements = vec![Spanned::new(Element::Other(format!("{}:", start_label)), loop_span)];
	elements.push(instruction!(Advance, "pass".to_owned(), loop_span));
	elements.push(instruction!(Advance, Reverse, format!("jump {}", end_label), loop_span));
	elements
}

pub fn loop_end_condition(mut elements: Vec<Spanned<Element>>, context: &mut FunctionContext,
                          condition: &Spanned<Expression>, end_label: usize) -> Vec<Spanned<Element>> {
	super::polarize(&mut elements, Direction::Advance);
	let expression_index = context.pop_evaluation().promote(&mut elements, context);
	let instruction = format!("branch.i = {} true {}", expression_index, end_label);
	elements.push(instruction!(Advance, Advance, instruction, condition.span));
	elements
}

pub fn loop_start_condition(mut elements: Vec<Spanned<Element>>, context: &mut FunctionContext,
                            condition: &Spanned<Expression>, start_label: usize) -> Vec<Spanned<Element>> {
	super::polarize_reverse(&mut elements);
	let expression_index = context.pop_evaluation().promote(&mut elements, context);
	let instruction = format!("branch.i = {} true {}", expression_index, start_label);
	elements.push(instruction!(Advance, Reverse, instruction, condition.span));
	elements
}

pub fn loop_suffix(loop_span: Span, start_label: usize, end_label: usize) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	elements.push(instruction!(Advance, Advance, format!("jump {}", start_label), loop_span));
	elements.push(Spanned::new(Element::Other(format!("{}:", end_label)), loop_span));
	elements.push(instruction!(Advance, "pass".to_owned(), loop_span));
	elements
}
