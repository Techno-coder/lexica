use std::collections::HashMap;

use super::{CompilationUnit, Instruction, InstructionTarget, TranslationFunctionLabel};

const ENTRY_POINT: &'static str = "main";

#[derive(Debug, Default)]
pub struct TranslationUnit {
	pub instructions: Vec<Instruction>,
	pub functions: HashMap<String, TranslationFunctionLabel>,
	pub labels: HashMap<String, (InstructionTarget, HashMap<String, InstructionTarget>)>,
}

impl TranslationUnit {
	pub fn compile(self) -> CompilationUnit {
		let mut main = None;
		let mut function_labels = HashMap::new();
		let mut reverse_labels = HashMap::new();
		for (name, function_label) in self.functions {
			if name == ENTRY_POINT {
				main = Some(function_label.target.clone());
			}

			if let Some(reverse_label) = function_label.reverse_target.clone() {
				reverse_labels.insert(reverse_label, function_label.target.clone());
			}
			function_labels.insert(function_label.target.clone(), function_label.compile());
		}

		CompilationUnit {
			instructions: self.instructions,
			function_labels,
			reverse_labels,
			main,
		}
	}
}
