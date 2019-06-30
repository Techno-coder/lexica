use std::collections::HashMap;

use crate::interpreter::Size;
use crate::node::Identifier;
use crate::source::{Span, Spanned};

use super::Evaluation;

#[derive(Debug, Default)]
pub struct FunctionContext<'a> {
	label_index: usize,
	local_sizes: Vec<Size>,
	evaluation_stack: Vec<Evaluation>,
	identifier_table: HashMap<Identifier<'a>, (usize, Span)>,
}

impl<'a> FunctionContext<'a> {
	pub fn register_variable(&mut self, identifier: Spanned<Identifier<'a>>, size: Size) -> usize {
		let local_index = self.register_local(size);
		self.annotate_local(local_index, identifier);
		local_index
	}

	pub fn get_variable(&self, variable: &Identifier<'a>) -> usize {
		let (local_index, _) = &self.identifier_table[variable];
		*local_index
	}

	pub fn drop_variable(&mut self, variable: &Identifier<'a>) -> usize {
		let (local_index, _) = self.identifier_table.remove(&variable).unwrap();
		self.identifier_table.remove(&variable);
		local_index
	}

	pub fn register_local(&mut self, size: Size) -> usize {
		self.local_sizes.push(size);
		self.local_sizes.len() - 1
	}

	pub fn clone_local(&mut self, local_index: usize) -> usize {
		self.register_local(self.local_sizes[local_index].clone())
	}

	pub fn annotate_local(&mut self, local_index: usize, identifier: Spanned<Identifier<'a>>) {
		self.identifier_table.insert(identifier.node, (local_index, identifier.span));
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

	pub fn identifier_table(&self) -> impl Iterator<Item=(&Identifier, &(usize, Span))> {
		self.identifier_table.iter()
	}
}
