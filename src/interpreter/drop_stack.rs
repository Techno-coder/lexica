use std::collections::VecDeque;

use super::{InterpreterError, InterpreterResult};

#[derive(Debug, Default)]
pub struct DropStack {
	stack: VecDeque<u8>,
}

impl DropStack {
	pub fn push_byte(&mut self, byte: u8) {
		self.stack.push_back(byte);
	}

	pub fn pop_byte(&mut self) -> InterpreterResult<u8> {
		self.stack.pop_back().ok_or(InterpreterError::DropStackEmpty)
	}
}
