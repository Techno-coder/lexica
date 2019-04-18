use std::fmt;

use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive, Size};

#[derive(Debug)]
pub struct Drop {
	local: LocalTarget,
}

impl Drop {
	pub fn new(table: &LocalTable, local: LocalTarget) -> InterpreterResult<Drop> {
		let _local = table.local(&local)?;
		Ok(Drop { local })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		let local = context.frame()?.table()[&self.local].clone();
		local.drop(context.drop_stack());
		Ok(())
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		let mut local = context.frame()?.table()[&self.local].clone();
		local.restore(context.drop_stack())?;
		context.frame()?.table_mut()[&self.local] = local;
		Ok(())
	}
}

impl fmt::Display for Drop {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.local)
	}
}

#[derive(Debug)]
pub struct DropImmediate {
	immediate: Primitive,
}

impl DropImmediate {
	pub fn new(size: Size, immediate: Primitive) -> InterpreterResult<DropImmediate> {
		match immediate.cast(size) {
			Some(immediate) => Ok(DropImmediate { immediate }),
			None => Err(InterpreterError::TypesIncompatible),
		}
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		Ok(self.immediate.drop(context.drop_stack()))
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		let byte_count = self.immediate.size().byte_count();
		for _ in 0..byte_count {
			context.drop_stack().pop_byte()?;
		}
		Ok(())
	}
}

impl fmt::Display for DropImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.immediate)
	}
}
