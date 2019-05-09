use std::collections::HashMap;

use crate::source::Spanned;

use super::{Instruction, LocalTable, TranslationInstruction};

#[derive(Debug, Default)]
pub struct Function {
	pub locals: LocalTable,
	pub instructions: Vec<Instruction>,
}

impl Function {
	pub fn instruction(&self, target: FunctionOffset) -> Option<&Instruction> {
		let FunctionOffset(index) = target;
		self.instructions.get(index)
	}
}

/// An index for a function within a unit.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionTarget(pub usize);

/// An index for an instruction within a function.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionOffset(pub usize);

#[derive(Debug, Default)]
pub struct TranslationFunction<'a> {
	pub locals: LocalTable,
	pub instructions: Vec<Spanned<TranslationInstruction<'a>>>,
	pub labels: HashMap<String, FunctionOffset>,
}
