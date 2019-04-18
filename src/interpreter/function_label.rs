use super::{InstructionTarget, LocalTable};

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

impl TranslationFunctionLabel {
	pub fn compile(self) -> FunctionLabel {
		FunctionLabel {
			locals: self.locals,
		}
	}
}

#[derive(Debug)]
pub struct FunctionLabelIndex(pub usize);
