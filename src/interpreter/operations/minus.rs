use std::fmt;

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

impl fmt::Display for Minus {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
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

impl fmt::Display for MinusImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
	}
}
