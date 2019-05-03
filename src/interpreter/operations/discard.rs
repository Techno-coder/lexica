use std::fmt;

use super::{Context, InterpreterResult, Size, Operation, CompilationUnit};

#[derive(Debug)]
pub struct Discard {
	size: Size,
}

impl Discard {
	pub fn new(size: Size) -> Discard {
		Discard { size }
	}
}

impl Operation for Discard {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let byte_count = self.size.byte_count();
		for _ in 0..byte_count {
			context.drop_stack().pop_byte()?;
		}
		Ok(())
	}
}

impl fmt::Display for Discard {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{}", self.size)
	}
}
