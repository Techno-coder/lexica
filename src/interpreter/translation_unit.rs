use std::collections::HashMap;

use super::{Instruction, InstructionTarget, TranslationFunctionLabel};

pub struct TranslationUnit {
	instructions: Vec<Instruction>,
	functions: HashMap<String, TranslationFunctionLabel>,
	labels: HashMap<String, InstructionTarget>,
}
