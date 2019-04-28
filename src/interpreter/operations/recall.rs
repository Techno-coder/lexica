use std::fmt;

use super::{CallFrame, CompilationUnit, Context, Direction, InstructionTarget};

#[derive(Debug)]
pub struct Recall {
	target: InstructionTarget,
	reverse_target: InstructionTarget,
}

impl Recall {
	pub fn new(target: InstructionTarget, reverse_target: InstructionTarget) -> Recall {
		Recall { target, reverse_target }
	}

	pub fn execute(&self, context: &mut Context, unit: &CompilationUnit) {
		let label = unit.reverse_labels.get(&self.reverse_target)
			.expect("Reverse function label does not exist");
		let function = unit.function_labels.get(label).unwrap();
		context.push_frame(CallFrame::construct(&function, Direction::Reverse, context.program_counter()));
		context.set_next_instruction(|| Ok(self.reverse_target.clone()));
	}

	pub fn reverse(&self, context: &mut Context, unit: &CompilationUnit) {
		let function = unit.function_labels.get(&self.target)
			.expect("Function label does not exist");
		context.push_frame(CallFrame::construct(&function, Direction::Reverse, context.program_counter()));
		context.set_next_instruction(|| Ok(self.target.clone()));
	}
}

impl fmt::Display for Recall {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self.target)
	}
}
