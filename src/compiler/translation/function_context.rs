use hashbrown::HashMap;

use crate::interpreter::Size;
use crate::node::VariableTarget;
use crate::source::{Span, Spanned};

use super::Evaluation;

/// Stores the local index and span of locals associated with a named variable.
type VariableFrame<'a> = HashMap<VariableTarget<'a>, (usize, Span)>;
/// Stores unnamed locals that must be dropped.
type IntermediateFrame<'a> = HashMap<usize, Span>;

#[derive(Debug, Default)]
pub struct FunctionContext<'a> {
	label_index: usize,
	local_sizes: Vec<Size>,
	evaluation_stack: Vec<Evaluation>,
	variable_stack: Vec<VariableFrame<'a>>,
	intermediate_stack: Vec<IntermediateFrame<'a>>,
}

impl<'a> FunctionContext<'a> {
	pub fn register_variable(&mut self, target: Spanned<VariableTarget<'a>>, size: Size) -> usize {
		let local_index = self.register_local(size);
		self.annotate_local(local_index, target);
		local_index
	}

	pub fn get_variable(&self, target: &VariableTarget<'a>) -> usize {
		for frame in self.variable_stack.iter().rev() {
			if let Some((local_index, _)) = frame.get(target) {
				return *local_index;
			}
		}
		panic!("Variable target: {}, is not bound", target)
	}

	pub fn register_local(&mut self, size: Size) -> usize {
		self.local_sizes.push(size);
		self.local_sizes.len() - 1
	}

	pub fn drop_variable(&mut self, target: &VariableTarget<'a>) -> usize {
		let (local_index, _) = self.variable_frame().remove(&target).unwrap();
		self.variable_frame().remove(&target);
		local_index
	}

	pub fn clone_local(&mut self, local_index: usize) -> usize {
		self.register_local(self.local_sizes[local_index].clone())
	}

	/// Associates an identifier with a local.
	/// If the local exists as an intermediate it is promoted to a variable.
	pub fn annotate_local(&mut self, local_index: usize, target: Spanned<VariableTarget<'a>>) {
		self.variable_frame().insert(target.node, (local_index, target.span));
		self.drop_intermediate(&local_index);
	}

	pub fn register_intermediate(&mut self, local_index: usize, span: Span) {
		self.intermediate_frame().insert(local_index, span);
	}

	/// Drops the intermediate from the current frame if it exists.
	pub fn drop_intermediate(&mut self, local_index: &usize) {
		self.intermediate_frame().remove(local_index);
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
		self.intermediate_stack.push(IntermediateFrame::new());
	}

	pub fn pop_frame(&mut self) -> (VariableFrame, IntermediateFrame) {
		let variable_frame = self.variable_stack.pop()
			.expect("Variable frame stack is empty");
		let intermediate_frame = self.intermediate_stack.pop()
			.expect("Intermediate frame stack is empty");
		(variable_frame, intermediate_frame)
	}

	fn variable_frame(&mut self) -> &mut VariableFrame<'a> {
		self.variable_stack.last_mut().expect("Variable frame stack is empty")
	}

	fn intermediate_frame(&mut self) -> &mut IntermediateFrame<'a> {
		self.intermediate_stack.last_mut().expect("Intermediate frame stack is empty")
	}
}
