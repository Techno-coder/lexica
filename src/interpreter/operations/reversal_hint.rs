use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, Direction, GenericOperation, InstructionTarget, InterpreterResult,
            Operand, Operation, Operational, ParserContext, ParserResult, TranslationUnit};

#[derive(Debug)]
pub struct ReversalHint;

impl Operational for ReversalHint {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		Ok(Box::new(ReversalHint))
	}
}

impl Operation for ReversalHint {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let is_halted = context.is_halted();
		context.set_is_halted(!is_halted);
		Ok(())
	}

	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.execute(context, unit)
	}
}

impl fmt::Display for ReversalHint {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
