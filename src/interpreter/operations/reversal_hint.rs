use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterResult,
            Operand, Operation, Operational, Reversible};

#[derive(Debug)]
pub struct ReversalHint;

impl Operational for ReversalHint {
	fn arity() -> usize { 0 }

	fn compile<'a, 'b>(_: Span, _: &[Operand<'a>], _: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		Ok(Box::new(ReversalHint))
	}
}

impl Operation for ReversalHint {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		context.is_trapped = !context.is_trapped;
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
