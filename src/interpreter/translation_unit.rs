use std::collections::HashMap;

use super::{Instruction, InstructionTarget, TranslationFunctionLabel};

#[derive(Debug, Default)]
pub struct TranslationUnit {
	pub instructions: Vec<Instruction>,
	pub functions: HashMap<String, TranslationFunctionLabel>,
	pub labels: HashMap<String, (InstructionTarget, HashMap<String, InstructionTarget>)>,
}
