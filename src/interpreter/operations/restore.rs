use std::fmt;

use super::{Context, Drop, InterpreterResult, LocalTable, LocalTarget, Operation, CompilationUnit};

#[derive(Debug)]
pub struct Restore {
	inner: Drop,
}

impl Restore {
	pub fn new(table: &LocalTable, local: LocalTarget) -> InterpreterResult<Restore> {
		let inner = Drop::new(table, local)?;
		Ok(Restore { inner })
	}

	pub fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.reverse(context, unit)
	}

	pub fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.execute(context, unit)
	}
}

impl fmt::Display for Restore {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
	}
}
