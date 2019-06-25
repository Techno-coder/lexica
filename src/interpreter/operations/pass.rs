use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation,
            InterpreterResult, Operand, Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Pass;

impl Operational for Pass {
	fn arity() -> usize { 0 }

	fn compile<'a>(_: Span, _: &[Operand<'a>], _: &CompileContext) -> CompileResult<'a, GenericOperation> {
		Ok(Box::new(Pass))
	}
}

impl Operation for Pass {
	fn execute(&self, _: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Pass {
	fn reverse(&self, _: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		Ok(())
	}
}

impl fmt::Display for Pass {
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
