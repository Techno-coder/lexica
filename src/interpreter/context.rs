use super::{CallFrame, DropStack, InstructionTarget, InterpreterError, InterpreterResult};
use crate::interpreter::Direction;

#[derive(Debug)]
pub struct Context {
	pub is_halted: bool,
	pub is_trapped: bool,
	pub step_direction: Direction,
	call_stack: Vec<CallFrame>,
	drop_stack: DropStack,
	program_counter: InstructionTarget,
	next_instruction: Option<InterpreterResult<InstructionTarget>>,
}

impl Context {
	pub fn new(program_counter: InstructionTarget) -> Context {
		Context {
			is_halted: false,
			is_trapped: false,
			call_stack: Vec::new(),
			drop_stack: DropStack::default(),
			step_direction: Direction::Advance,
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
	pub fn set_next_instruction<F>(&mut self, functor: F)
		where F: FnOnce() -> InterpreterResult<InstructionTarget> {
		if self.next_instruction.is_none() {
			self.next_instruction = Some(functor());
		}
	}

	pub fn program_counter(&self) -> InstructionTarget {
		self.program_counter.clone()
	}

	pub fn advance(&mut self) -> InterpreterResult<()> {
		match self.next_instruction.take() {
			Some(instruction) => Ok(self.program_counter = instruction?),
			None => Err(InterpreterError::NextInstructionNull),
		}
	}
}
