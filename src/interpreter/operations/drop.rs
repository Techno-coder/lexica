use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterResult,
            LocalTable, LocalTarget, Operand, Operation, Operational, Reverser, Reversible};

pub type Restore = Reverser<Drop>;

/// Pushes a local to the drop stack and zeroes it.
#[derive(Debug)]
pub struct Drop {
	local: LocalTarget,
}

impl Drop {
	pub fn new(table: &LocalTable, local: LocalTarget) -> InterpreterResult<Drop> {
		let _local = table.local(&local)?;
		Ok(Drop { local })
	}
}

impl Operational for Drop {
	fn arity() -> usize { 1 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let local = local(&operands[0])?;
		let table = local_table(&base_function(context, span));
		Ok(Box::new(error(Drop::new(table?, local), span)?))
	}
}

impl Operation for Drop {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let local = context.frame()?.table()[&self.local].clone();
		local.drop(context.drop_stack());
		context.frame()?.table_mut()[&self.local] = local.size().primitive();
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Drop {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let mut local = context.frame()?.table()[&self.local].clone();
		local.restore(context.drop_stack())?;
		context.frame()?.table_mut()[&self.local] = local;
		Ok(())
	}
}

impl fmt::Display for Drop {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.local)
	}
}
