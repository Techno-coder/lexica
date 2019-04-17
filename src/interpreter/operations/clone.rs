use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget};

#[derive(Debug)]
pub struct Clone {
	left: LocalTarget,
	right: LocalTarget,
}

impl Clone {
	pub fn new(table: &LocalTable, left: LocalTarget, right: LocalTarget)
	           -> InterpreterResult<Clone> {
		let left_size = table.local(&left)?.size().byte_count();
		let right_size = table.local(&right)?.size().byte_count();
		if left_size == right_size {
			Ok(Clone { left, right })
		} else {
			Err(InterpreterError::SizeIncompatible)
		}
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		use std::mem;
		let table = context.frame()?.table_mut();
		let right = table[&self.right].clone();
		mem::replace(&mut table[&self.left], right);
		Ok(())
	}
}