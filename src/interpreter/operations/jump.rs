use std::fmt;

use super::{Context, InstructionTarget, Operation, CompilationUnit, InterpreterResult};

#[derive(Debug)]
pub struct Jump {
	target: InstructionTarget,
}

impl Jump {
	pub fn new(target: InstructionTarget) -> Jump {
		Jump { target }
	}
}

impl Operation for Jump {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		context.set_program_counter(self.target.clone());
		Ok(())
	}
}

impl fmt::Display for Jump {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self.target)
	}
}
