use super::{CallFrame, DropStack, InstructionTarget, InterpreterError, InterpreterResult};

#[derive(Debug)]
pub struct Context {
	call_stack: Vec<CallFrame>,
	drop_stack: DropStack,
	program_counter: usize,
}

impl Context {
	pub fn new(program_counter: usize) -> Context {
		Context {
			call_stack: Vec::new(),
			drop_stack: DropStack::default(),
			program_counter,
		}
	}

	pub fn frame(&mut self) -> InterpreterResult<&mut CallFrame> {
		self.call_stack.last_mut().ok_or(InterpreterError::CallStackEmpty)
	}

	pub fn drop_stack(&mut self) -> &mut DropStack {
		&mut self.drop_stack
	}

	pub fn set_next_instruction(&mut self, target: &InstructionTarget) {
		let InstructionTarget(counter) = target;
		self.program_counter = *counter;
	}
}
