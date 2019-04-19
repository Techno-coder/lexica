use std::fmt;

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
		context.set_program_counter(self.target.clone())
	}
}

impl fmt::Display for Jump {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{:?}", self.target)
	}
}
