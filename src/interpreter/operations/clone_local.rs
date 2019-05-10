use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterError,
            InterpreterResult, LocalTable, LocalTarget, Operand, Operation, Operational};

#[derive(Debug)]
pub struct CloneLocal {
	left: LocalTarget,
	right: LocalTarget,
}

impl CloneLocal {
	pub fn new(table: &LocalTable, left: LocalTarget, right: LocalTarget)
	           -> InterpreterResult<CloneLocal> {
		let left_size = table.local(&left)?.size().byte_count();
		let right_size = table.local(&right)?.size().byte_count();
		if left_size == right_size {
			Ok(CloneLocal { left, right })
		} else {
			Err(InterpreterError::SizeIncompatible)
		}
	}
}

impl Operational for CloneLocal {
	fn compile<'a, 'b>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, span));
		let (left, right) = (local(&operands[0])?, local(&operands[1])?);
		Ok(Box::new(error(CloneLocal::new(table?, left, right), span)?))
	}
}

impl Operation for CloneLocal {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		use std::mem;
		let table = context.frame()?.table_mut();
		let right = table[&self.right].clone();
		mem::replace(&mut table[&self.left], right);
		Ok(())
	}
}

impl fmt::Display for CloneLocal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.left, self.right)
	}
}
