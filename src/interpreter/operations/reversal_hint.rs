use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterResult, Operand, Operation,
            Operational, ParserContext, ParserResult, Reversible, TranslationUnit};

#[derive(Debug)]
pub struct ReversalHint;

impl Operational for ReversalHint {
	fn parse<'a>(_: &Span, _: &Vec<Operand<'a>>, _: &ParserContext,
	             _: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		Ok(Box::new(ReversalHint))
	}
}

impl Operation for ReversalHint {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let is_halted = context.is_halted();
		context.set_is_halted(!is_halted);
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for ReversalHint {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.execute(context, unit)
	}
}

impl fmt::Display for ReversalHint {
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
