use super::{Context, Drop, InterpreterResult, LocalTable, LocalTarget};

#[derive(Debug)]
pub struct Restore {
	inner: Drop,
}

impl Restore {
	pub fn new(table: &LocalTable, local: LocalTarget) -> InterpreterResult<Restore> {
		let inner = Drop::new(table, local)?;
		Ok(Restore { inner })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.reverse(context)
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.execute(context)
	}
}
