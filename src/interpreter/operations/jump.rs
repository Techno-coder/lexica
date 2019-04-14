use super::{Context, InstructionTarget};

#[derive(Debug)]
pub struct Jump {
	target: InstructionTarget,
}

impl Jump {
	pub fn new(target: InstructionTarget) -> Jump {
		Jump { target }
	}

	pub fn execute(&self, context: &mut Context) {
		context.set_next_instruction(&self.target)
	}
}
