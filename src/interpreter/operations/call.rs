use super::{Context, InstructionTarget};

#[derive(Debug)]
pub struct Call {
	target: InstructionTarget,
}

impl Call {
	pub fn new(target: InstructionTarget) -> Call {
		Call { target }
	}

	pub fn execute(&self, context: &mut Context) {
		context.set_next_instruction(&self.target)
	}
}
