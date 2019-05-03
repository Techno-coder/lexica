use std::fmt;

use super::{Add, AddImmediate, CompilationUnit, Context, InterpreterResult, LocalTable, LocalTarget,
            Operation, Primitive};

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
}

impl Operation for Minus {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.reverse(context, unit)
	}

	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.execute(context, unit)
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
}

impl Operation for MinusImmediate {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.reverse(context, unit)
	}

	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.inner.execute(context, unit)
	}
}

impl fmt::Display for MinusImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.inner)
	}
}
