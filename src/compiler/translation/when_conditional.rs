use crate::interpreter::Direction;
use crate::source::Spanned;

use super::{Element, FunctionContext};

type WhenConditional<'a, 'b> = &'b mut Spanned<crate::node::WhenConditional<'a>>;

pub fn when_branch_labels(when_conditional: WhenConditional, context: &mut FunctionContext)
                          -> Vec<(usize, usize)> {
	let mut split_label = None;
	when_conditional.branches.iter()
		.map(|_| {
			let (entry, exit) = match split_label {
				None => context.pair_labels(),
				Some(entry) => (entry, context.label()),
			};

			split_label = Some(exit);
			(entry, exit)
		}).collect()
}

pub fn when_entry(when_conditional: WhenConditional, branch_labels: &Vec<(usize, usize)>,
                  conditions: Vec<Vec<Spanned<Element>>>, context: &mut FunctionContext,
                  start_label: usize, end_label: usize) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let start_label_element = Element::Other(format!("{}: pass", start_label));
	elements.push(Spanned::new(start_label_element, when_conditional.span));

	let mut evaluations: Vec<_> = conditions.iter().map(|_| context.pop_evaluation()).collect();
	for (branch_index, mut condition_elements) in conditions.into_iter().enumerate() {
		let evaluation = evaluations.pop().unwrap();
		let expression_index = evaluation.promote(&mut condition_elements, context);

		let (branch_entry, _) = &branch_labels[branch_index];
		let instruction = format!("branch.i = {} true {}", expression_index, branch_entry);
		let span = when_conditional.branches[branch_index].condition.span;
		condition_elements.push(instruction!(Advance, Advance, instruction, span));

		super::polarize(&mut condition_elements, Direction::Advance);
		elements.append(&mut condition_elements);
	}

	let instruction = format!("jump {}", end_label);
	elements.push(instruction!(Advance, Advance, instruction, when_conditional.span));
	elements
}

pub fn when_expressions(when_conditional: WhenConditional, branch_labels: &Vec<(usize, usize)>,
                        expressions: Vec<Vec<Spanned<Element>>>, start_label: usize, end_label: usize)
                        -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	for (branch_index, mut expression_elements) in expressions.into_iter().enumerate() {
		let span = when_conditional.branches[branch_index].expression_block.span;
		let (branch_entry, branch_exit) = &branch_labels[branch_index];

		if branch_index == 0 {
			let branch_entry_element = Element::Other(format!("{}: pass", branch_entry));
			elements.push(Spanned::new(branch_entry_element, span));
		}

		elements.push(instruction!(Advance, Reverse, format!("jump {}", start_label), span));
		elements.append(&mut expression_elements);
		elements.push(instruction!(Advance, Advance, format!("jump {}", end_label), span));

		let branch_exit_element = Element::Other(format!("{}: pass", branch_exit));
		elements.push(Spanned::new(branch_exit_element, span));
	}
	elements
}

pub fn when_reverse_entry(when_conditional: WhenConditional, branch_labels: &Vec<(usize, usize)>,
                          end_conditions: Vec<Vec<Spanned<Element>>>, context: &mut FunctionContext,
                          start_label: usize, end_label: usize) -> Vec<Spanned<Element>> {
	let mut elements = Vec::new();
	let instruction = format!("jump {}", start_label);
	elements.push(instruction!(Advance, Reverse, instruction, when_conditional.span));

	for (branch_index, mut condition_elements) in end_conditions.into_iter().rev().enumerate() {
		let evaluation = context.pop_evaluation();
		let expression_index = evaluation.promote(&mut condition_elements, context);

		let (_, branch_exit) = &branch_labels[branch_index];
		let instruction = format!("branch.i = {} true {}", expression_index, branch_exit);
		let span = when_conditional.branches[branch_index].end_condition.as_ref().unwrap().span;
		condition_elements.push(instruction!(Advance, Reverse, instruction, span));

		super::polarize_reverse(&mut condition_elements);
		elements.append(&mut condition_elements);
	}

	let end_label_element = Element::Other(format!("{}: pass", end_label));
	elements.push(Spanned::new(end_label_element, when_conditional.span));
	elements
}
