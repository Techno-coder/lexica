use super::{Context, InstructionTarget, InterpreterError, InterpreterResult};

#[derive(Debug)]
pub struct Call {
	target: InstructionTarget,
	reverse_target: Option<InstructionTarget>,
}

impl Call {
	pub fn new(target: InstructionTarget, reverse_target: Option<InstructionTarget>) -> Call {
		Call { target, reverse_target }
	}

	pub fn execute(&self, context: &mut Context) {
		context.set_next_instruction(&self.target)
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		let reverse_target = self.reverse_target.as_ref()
		                         .ok_or(InterpreterError::Irreversible)?;
		context.set_next_instruction(&reverse_target);
		Ok(())
	}

	pub fn reversible(&self) -> bool {
		self.reverse_target.is_some()
	}
}

#[derive(Debug)]
pub struct Return;

impl Return {
	pub fn execute(&self, context: &mut Context) {
		unimplemented!()
	}
}
