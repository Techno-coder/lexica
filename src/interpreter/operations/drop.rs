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
		Ok(local.drop(context.drop_stack()))
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		let local = &mut context.frame()?.table_mut()[&self.local].clone();
		local.restore(context.drop_stack())
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
