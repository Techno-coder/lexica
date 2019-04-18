use std::collections::HashMap;

use super::{FunctionLabel, Instruction, InstructionTarget};

#[derive(Debug)]
pub struct CompilationUnit {
	pub instructions: Vec<Instruction>,
	pub function_labels: HashMap<InstructionTarget, FunctionLabel>,
	pub reverse_labels: HashMap<InstructionTarget, InstructionTarget>,
	pub main: Option<InstructionTarget>,
}
