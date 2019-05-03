use std::fmt;

use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive,
            Operation, CompilationUnit};

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
}

impl Operation for Reset {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		use std::mem;
		let local = &mut context.frame()?.table_mut()[&self.local];
		mem::replace(local, self.immediate.clone());
		Ok(())
	}
}

impl fmt::Display for Reset {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} {}", self.local, self.immediate)
	}
}
