use std::fmt;

use crate::source::Span;

use super::{Comparator, CompilationUnit, Context, GenericOperation, InstructionTarget, InterpreterResult,
            LocalTable, LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult,
            TranslationUnit};

#[derive(Debug)]
pub struct Branch {
	comparator: Comparator,
	left: LocalTarget,
	right: LocalTarget,
	target: InstructionTarget,
}

impl Branch {
	pub fn new(table: &LocalTable, comparator: Comparator, left: LocalTarget, right: LocalTarget,
	           target: InstructionTarget) -> InterpreterResult<Branch> {
		let left_local = table.local(&left)?;
		let right_local = table.local(&right)?;
		let _comparison = comparator.compare(left_local, right_local)?;
		Ok(Branch { comparator, left, right, target })
	}
}

impl Operational for Branch {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, unit, span));
		let comparator = comparator(&operands[0])?;
		let (left, right) = (local(&operands[1])?, local(&operands[2])?);
		let target = target_label(span, &operands[3], unit, context)?;
		Ok(Box::new(error(Branch::new(table?, comparator, left, right, target), span)?))
	}
}

impl Operation for Branch {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table();
		let left_local = &table[&self.left];
		let right_local = &table[&self.right];
		let comparison = self.comparator.compare(left_local, right_local)?;
		if comparison == true {
			context.set_next_instruction(|| Ok(self.target.clone()));
		}
		Ok(())
	}
}

impl fmt::Display for Branch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {} {:?}", self.comparator, self.left, self.right, self.target)
	}
}
