use super::{InstructionTarget, Primitive};

#[derive(Debug)]
pub struct FunctionLabel {
	pub locals: Vec<Primitive>,
}

#[derive(Debug)]
pub struct TranslationFunctionLabel {
	pub locals: Vec<Primitive>,
	pub target: InstructionTarget,
	pub reverse_target: Option<InstructionTarget>,
}

#[derive(Debug)]
pub struct FunctionLabelIndex(pub usize);
