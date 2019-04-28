use std::fmt;

use super::{CallFrame, CompilationUnit, Context, Direction, InstructionTarget,
            InterpreterError, InterpreterResult};

#[derive(Debug)]
pub struct Call {
	target: InstructionTarget,
	reverse_target: Option<InstructionTarget>,
}

impl Call {
	pub fn new(target: InstructionTarget, reverse_target: Option<InstructionTarget>) -> Call {
		Call { target, reverse_target }
	}

	pub fn execute(&self, context: &mut Context, unit: &CompilationUnit) {
		let function = unit.function_labels.get(&self.target)
			.expect("Function label does not exist");
		context.push_frame(CallFrame::construct(&function, Direction::Advance, context.program_counter()));
		context.set_next_instruction(|| Ok(self.target.clone()));
	}

	pub fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let reverse_target = self.reverse_target.as_ref()
			.ok_or(InterpreterError::Irreversible)?;
		let label = unit.reverse_labels.get(reverse_target)
			.expect("Reverse function label does not exist");
		let function = unit.function_labels.get(label).unwrap();
		context.push_frame(CallFrame::construct(&function, Direction::Advance, context.program_counter()));
		context.set_next_instruction(|| Ok(reverse_target.clone()));
		Ok(())
	}

	pub fn reversible(&self) -> bool {
		self.reverse_target.is_some()
	}
}

impl fmt::Display for Call {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self.target)
	}
}
