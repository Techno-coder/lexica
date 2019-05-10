use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterResult, Operand,
            Operation, Operational, Size};

#[derive(Debug)]
pub struct Discard {
	size: Size,
}

impl Discard {
	pub fn new(size: Size) -> Discard {
		Discard { size }
	}
}

impl Operational for Discard {
	fn compile<'a, 'b>(_: &Span, operands: &Vec<Operand<'a>>, _: &CompileContext)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let size = size(&operands[0])?;
		Ok(Box::new(Discard::new(size)))
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
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.size)
	}
}
