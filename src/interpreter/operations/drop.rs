use std::fmt;

use super::{CompilationUnit, Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget,
            Operation, Primitive, Size};

#[derive(Debug)]
pub struct Drop {
	local: LocalTarget,
}

impl Drop {
	pub fn new(table: &LocalTable, local: LocalTarget) -> InterpreterResult<Drop> {
		let _local = table.local(&local)?;
		Ok(Drop { local })
	}
}

impl Operation for Drop {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let local = context.frame()?.table()[&self.local].clone();
		local.drop(context.drop_stack());
		Ok(())
	}

	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
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
}

impl Operation for DropImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		Ok(self.immediate.drop(context.drop_stack()))
	}

	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
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
