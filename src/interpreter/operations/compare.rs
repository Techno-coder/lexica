use std::fmt;

use crate::source::Span;

use super::{Comparator, CompilationUnit, CompileContext, CompileResult, Context, GenericOperation,
            InterpreterError, InterpreterResult, LocalTable, LocalTarget, Operand, Operation,
            Operational, Primitive};

#[derive(Debug)]
pub struct Compare {
	comparator: Comparator,
	left: LocalTarget,
	right: LocalTarget,
	target: LocalTarget,
}

impl Compare {
	pub fn new(table: &LocalTable, comparator: Comparator, left: LocalTarget, right: LocalTarget,
	           target: LocalTarget) -> InterpreterResult<Compare> {
		let (left_local, right_local) = (table.local(&left)?, table.local(&right)?);
		let _comparison = comparator.compare(left_local, right_local)?;
		match table.local(&target)? {
			Primitive::Boolean(_) => Ok(Compare { comparator, left, right, target }),
			_ => Err(InterpreterError::NonBoolean),
		}
	}
}

impl Operational for Compare {
	fn arity() -> usize { 4 }

	fn compile<'a, 'b>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let function = base_function(context, span);
		let table = local_table(&function);
		let comparator = comparator(&operands[0])?;
		let (left, right) = (local(&operands[1])?, local(&operands[2])?);
		let target = local(&operands[3])?;
		Ok(Box::new(error(Compare::new(table?, comparator, left, right, target), span)?))
	}
}

impl Operation for Compare {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let (left_local, right_local) = (&table[&self.left], &table[&self.right]);
		let comparison = self.comparator.compare(left_local, right_local)?;
		match &mut table[&self.target] {
			Primitive::Boolean(boolean) => *boolean = comparison,
			_ => return Err(InterpreterError::InvalidRuntime),
		}
		Ok(())
	}
}

impl fmt::Display for Compare {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {} {}", self.comparator, self.left, self.right, self.target)
	}
}
