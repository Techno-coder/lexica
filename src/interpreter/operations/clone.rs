use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget};

#[derive(Debug)]
pub struct CloneLocal {
	left: LocalTarget,
	right: LocalTarget,
}

impl CloneLocal {
	pub fn new(table: &LocalTable, left: LocalTarget, right: LocalTarget)
	           -> InterpreterResult<CloneLocal> {
		let left_size = table.local(&left)?.size().byte_count();
		let right_size = table.local(&right)?.size().byte_count();
		if left_size == right_size {
			Ok(CloneLocal { left, right })
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