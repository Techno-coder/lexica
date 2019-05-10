use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterError,
            InterpreterResult, LocalTable, LocalTarget, Operand, Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Swap {
	left: LocalTarget,
	right: LocalTarget,
}

impl Swap {
	pub fn new(table: &LocalTable, left: LocalTarget, right: LocalTarget)
	           -> InterpreterResult<Swap> {
		let left_size = table.local(&left)?.size().byte_count();
		let right_size = table.local(&right)?.size().byte_count();
		if left_size == right_size {
			Ok(Swap { left, right })
		} else {
			Err(InterpreterError::SizeIncompatible)
		}
	}
}

impl Operational for Swap {
	fn arity() -> usize { 2 }

	fn compile<'a, 'b>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, span));
		let (left, right) = (local(&operands[0])?, local(&operands[1])?);
		Ok(Box::new(error(Swap::new(table?, left, right), span)?))
	}
}

impl Operation for Swap {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		use std::mem;
		let table = context.frame()?.table_mut();
		let left = table[&self.left].clone();
		let right = table[&self.right].clone();
		mem::replace(&mut table[&self.left], right);
		mem::replace(&mut table[&self.right], left);
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Swap {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.execute(context, unit)
	}
}

impl fmt::Display for Swap {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.left, self.right)
	}
}
