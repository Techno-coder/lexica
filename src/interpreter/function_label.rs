use super::{InstructionTarget, Primitive, LocalTable};

#[derive(Debug)]
pub struct FunctionLabel {
	pub locals: LocalTable,
}

#[derive(Debug)]
pub struct TranslationFunctionLabel {
	pub locals: LocalTable,
	pub target: InstructionTarget,
	pub reverse_target: Option<InstructionTarget>,
}

#[derive(Debug)]
pub struct FunctionLabelIndex(pub usize);
