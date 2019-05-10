use std::collections::HashMap;

use super::{Function, FunctionTarget, Instruction, InstructionTarget, TranslationFunction};

#[derive(Debug, Default)]
pub struct CompilationUnit {
	pub functions: Vec<Function>,
	pub main: Option<FunctionTarget>,
}

impl CompilationUnit {
	pub fn function(&self, target: FunctionTarget) -> Option<&Function> {
		let FunctionTarget(index) = target;
		self.functions.get(index)
	}

	pub fn instruction(&self, target: InstructionTarget) -> Option<&Instruction> {
		let InstructionTarget(target, offset) = target;
		self.function(target).and_then(|function| function.instruction(offset))
	}
}

#[derive(Debug, Default)]
pub struct TranslationUnit<'a> {
	pub functions: HashMap<String, TranslationFunction<'a>>,
}
