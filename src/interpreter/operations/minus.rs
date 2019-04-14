use super::{Add, AddImmediate, Context, InterpreterResult, LocalTable, LocalTarget, Primitive};

#[derive(Debug)]
pub struct Minus {
	inner: Add,
}

impl Minus {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, operand: LocalTarget)
	           -> InterpreterResult<Minus> {
		let inner = Add::new(table, accumulator, operand)?;
		Ok(Minus { inner })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.reverse(context)
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.execute(context)
	}
}

#[derive(Debug)]
pub struct MinusImmediate {
	inner: AddImmediate,
}

impl MinusImmediate {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, operand: Primitive)
	           -> InterpreterResult<MinusImmediate> {
		let inner = AddImmediate::new(table, accumulator, operand)?;
		Ok(MinusImmediate { inner })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.reverse(context)
	}

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
		self.inner.execute(context)
	}
}
