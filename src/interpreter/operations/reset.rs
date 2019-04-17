use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive};

#[derive(Debug)]
pub struct Reset {
	local: LocalTarget,
	immediate: Primitive,
}

impl Reset {
	pub fn new(table: &LocalTable, local: LocalTarget, immediate: Primitive)
	           -> InterpreterResult<Reset> {
		let table_local = table.local(&local)?;
		match immediate.cast(table_local.size()) {
			Some(immediate) => Ok(Reset { local, immediate }),
			None => Err(InterpreterError::TypesIncompatible),
		}
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		use std::mem;
		let local = &mut context.frame()?.table_mut()[&self.local];
		mem::replace(local, self.immediate.clone());
		Ok(())
	}
}
