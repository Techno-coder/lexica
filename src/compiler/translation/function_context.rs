use hashbrown::HashMap;

use crate::interpreter::Size;
use crate::node::VariableTarget;
use crate::source::{Span, Spanned};

use super::Evaluation;

type VariableFrame<'a> = HashMap<VariableTarget<'a>, (usize, Span)>;

#[derive(Debug, Default)]
pub struct FunctionContext<'a> {
	label_index: usize,
	local_sizes: Vec<Size>,
	evaluation_stack: Vec<Evaluation>,
	variable_stack: Vec<VariableFrame<'a>>,
}

impl<'a> FunctionContext<'a> {
	pub fn register_variable(&mut self, target: Spanned<VariableTarget<'a>>, size: Size) -> usize {
		let local_index = self.register_local(size);
		self.annotate_local(local_index, target);
		local_index
	}

	pub fn get_variable(&self, target: &VariableTarget<'a>) -> usize {
		for frame in &self.variable_stack {
			if let Some((local_index, _)) = frame.get(target) {
				return *local_index;
			}
		}
		panic!("Variable target is not bound")
	}

	pub fn drop_variable(&mut self, target: &VariableTarget<'a>) -> usize {
		let (local_index, _) = self.frame().remove(&target).unwrap();
		self.frame().remove(&target);
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
		self.frame().insert(target.node, (local_index, target.span));
	}

	pub fn push_evaluation(&mut self, evaluation: Evaluation) {
		self.evaluation_stack.push(evaluation);
	}

	pub fn pop_evaluation(&mut self) -> Evaluation {
		self.evaluation_stack.pop().expect("Expression stack is empty")
	}

	pub fn label(&mut self) -> usize {
		self.label_index += 1;
		self.label_index - 1
	}

	pub fn pair_labels(&mut self) -> (usize, usize) {
		(self.label(), self.label())
	}

	pub fn local_sizes(&self) -> &[Size] {
		&self.local_sizes
	}

	pub fn push_frame(&mut self) {
		self.variable_stack.push(VariableFrame::new());
	}

	pub fn pop_frame(&mut self) -> VariableFrame {
		self.variable_stack.pop().expect("Variable frame stack is empty")
	}

	fn frame(&mut self) -> &mut VariableFrame<'a> {
		self.variable_stack.last_mut().unwrap()
	}
}
