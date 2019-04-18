use super::{CallFrame, DropStack, InstructionTarget, InterpreterError, InterpreterResult};

#[derive(Debug)]
pub struct Context {
	call_stack: Vec<CallFrame>,
	drop_stack: DropStack,
	program_counter: InstructionTarget,
	next_instruction: Option<InstructionTarget>,
}

impl Context {
	pub fn new(program_counter: InstructionTarget) -> Context {
		Context {
			call_stack: Vec::new(),
			drop_stack: DropStack::default(),
			program_counter,
			next_instruction: None,
		}
	}

	pub fn frame(&mut self) -> InterpreterResult<&mut CallFrame> {
		self.call_stack.last_mut().ok_or(InterpreterError::CallStackEmpty)
	}

	pub fn push_frame(&mut self, frame: CallFrame) {
		self.call_stack.push(frame);
	}

	pub fn pop_frame(&mut self) -> InterpreterResult<()> {
		self.call_stack.pop().ok_or(InterpreterError::CallStackEmpty)?;
		Ok(())
	}

	pub fn drop_stack(&mut self) -> &mut DropStack {
		&mut self.drop_stack
	}

	/// Sets the next instruction to advance to on the next interpreter step.
	/// If this instruction has already been set then it is **not** overwritten.
	pub fn set_next_instruction(&mut self, target: InstructionTarget) {
		if self.next_instruction.is_none() {
			self.next_instruction = Some(target);
		}
	}

	pub fn program_counter(&self) -> InstructionTarget {
		self.program_counter.clone()
	}

	pub fn advance(&mut self) -> InterpreterResult<()> {
		match self.next_instruction.take() {
			Some(instruction) => Ok(self.program_counter = instruction),
			None => Err(InterpreterError::NextInstructionNull),
		}
	}
}
