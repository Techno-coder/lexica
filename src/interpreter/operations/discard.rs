use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterResult, Operand, Operation, Operational,
            ParserContext, ParserResult, Size, TranslationUnit};

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
	fn parse<'a>(_: &Span, operands: &Vec<Operand<'a>>, _: &ParserContext,
	             _: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
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
