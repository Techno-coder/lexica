use std::fmt;

use crate::source::Span;

use super::{Comparator, CompilationUnit, Context, GenericOperation, InstructionTarget, InterpreterResult,
            LocalTable, LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult,
            Primitive, TranslationUnit};

#[derive(Debug)]
pub struct BranchImmediate {
	comparator: Comparator,
	local: LocalTarget,
	immediate: Primitive,
	target: InstructionTarget,
}

impl BranchImmediate {
	pub fn new(table: &LocalTable, comparator: Comparator, local: LocalTarget, immediate: Primitive,
	           target: InstructionTarget) -> InterpreterResult<BranchImmediate> {
		let table_local = table.local(&local)?;
		let _comparison = comparator.compare(table_local, &immediate)?;
		Ok(BranchImmediate { comparator, local, immediate, target })
	}
}

impl Operational for BranchImmediate {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, unit, span));
		let comparator = comparator(&operands[0])?;
		let (left, right) = (local(&operands[1])?, primitive(&operands[2])?);
		let target = target_label(span, &operands[3], unit, context)?;
		Ok(Box::new(error(BranchImmediate::new(table?, comparator, left, right, target), span)?))
	}
}

impl Operation for BranchImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table();
		let local = &table[&self.local];
		let comparison = self.comparator.compare(local, &self.immediate)?;
		if comparison == true {
			context.set_program_counter(self.target.clone());
		}
		Ok(())
	}
}

impl fmt::Display for BranchImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {} {:?}", self.comparator, self.local, self.immediate, self.target)
	}
}
