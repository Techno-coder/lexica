use std::fmt;

use super::{CompilationUnit, Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget,
            Operation};

#[derive(Debug)]
pub struct Swap {
	left: LocalTarget,
	right: LocalTarget,
}

impl Swap {
	pub fn new(table: &LocalTable, left: LocalTarget, right: LocalTarget)
	           -> InterpreterResult<Swap> {
		let left_size = table.local(&left)?.size().byte_count();
		let right_size = table.local(&right)?.size().byte_count();
		if left_size == right_size {
			Ok(Swap { left, right })
		} else {
			Err(InterpreterError::SizeIncompatible)
		}
	}
}

impl Operation for Swap {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		use std::mem;
		let table = context.frame()?.table_mut();
		let left = table[&self.left].clone();
		let right = table[&self.right].clone();
		mem::replace(&mut table[&self.left], right);
		mem::replace(&mut table[&self.right], left);
		Ok(())
	}
}

impl fmt::Display for Swap {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} {}", self.left, self.right)
	}
}