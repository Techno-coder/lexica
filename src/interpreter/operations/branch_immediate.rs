use std::fmt;

use crate::source::Span;

use super::{Comparator, CompilationUnit, CompileContext, CompileResult, Context, FunctionOffset,
            GenericOperation, InstructionTarget, InterpreterResult, LocalTable, LocalTarget, Operand,
            Operation, Operational, Primitive};

#[derive(Debug)]
pub struct BranchImmediate {
	comparator: Comparator,
	local: LocalTarget,
	immediate: Primitive,
	target: FunctionOffset,
}

impl BranchImmediate {
	pub fn new(table: &LocalTable, comparator: Comparator, local: LocalTarget, immediate: Primitive,
	           target: FunctionOffset) -> InterpreterResult<BranchImmediate> {
		let table_local = table.local(&local)?;
		let _comparison = comparator.compare(table_local, &immediate)?;
		Ok(BranchImmediate { comparator, local, immediate, target })
	}
}

impl Operational for BranchImmediate {
	fn arity() -> usize { 4 }

	fn compile<'a, 'b>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let function = base_function(context, span);
		let table = local_table(&function);
		let comparator = comparator(&operands[0])?;
		let (left, right) = (local(&operands[1])?, primitive(&operands[2])?);
		let target = target_label(span, &operands[3], function?)?;
		Ok(Box::new(error(BranchImmediate::new(table?, comparator, left, right, target), span)?))
	}
}

impl Operation for BranchImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table();
		let local = &table[&self.local];
		let comparison = self.comparator.compare(local, &self.immediate)?;
		if comparison == true {
			let InstructionTarget(function, _) = context.program_counter();
			context.set_next_instruction(|| Ok(InstructionTarget(function, self.target.clone())));
		}
		Ok(())
	}
}

impl fmt::Display for BranchImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {} {:?}", self.comparator, self.local, self.immediate, self.target)
	}
}
