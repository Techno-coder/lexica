use std::collections::HashMap;

use crate::interpreter::Size;
use crate::node::VariableTarget;
use crate::source::{Span, Spanned};

use super::Evaluation;

#[derive(Debug, Default)]
pub struct FunctionContext<'a> {
	label_index: usize,
	local_sizes: Vec<Size>,
	evaluation_stack: Vec<Evaluation>,
	variable_table: HashMap<VariableTarget<'a>, (usize, Span)>,
}

impl<'a> FunctionContext<'a> {
	pub fn register_variable(&mut self, target: Spanned<VariableTarget<'a>>, size: Size) -> usize {
		let local_index = self.register_local(size);
		self.annotate_local(local_index, target);
		local_index
	}

	pub fn get_variable(&self, target: &VariableTarget<'a>) -> usize {
		let (local_index, _) = &self.variable_table[target];
		*local_index
	}

	pub fn drop_variable(&mut self, target: &VariableTarget<'a>) -> usize {
		let (local_index, _) = self.variable_table.remove(&target).unwrap();
		self.variable_table.remove(&target);
		local_index
	}

	pub fn register_local(&mut self, size: Size) -> usize {
		self.local_sizes.push(size);
		self.local_sizes.len() - 1
	}

	pub fn clone_local(&mut self, local_index: usize) -> usize {
		self.register_local(self.local_sizes[local_index].clone())
	}

	pub fn annotate_local(&mut self, local_index: usize, target: Spanned<VariableTarget<'a>>) {
		self.variable_table.insert(target.node, (local_index, target.span));
	}

	pub fn push_evaluation(&mut self, evaluation: Evaluation) {
		self.evaluation_stack.push(evaluation);
	}

	pub fn pop_evaluation(&mut self) -> Evaluation {
		self.evaluation_stack.pop().expect("Expression stack is empty")
	}

	pub fn pair_labels(&mut self) -> (usize, usize) {
		let labels = (self.label_index, self.label_index + 1);
		self.label_index += 2;
		labels
	}

	pub fn local_sizes(&self) -> &[Size] {
		&self.local_sizes
	}

	pub fn variable_table(&self) -> impl Iterator<Item=(&VariableTarget, &(usize, Span))> {
		self.variable_table.iter()
	}
}
