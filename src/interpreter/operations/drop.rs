use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterResult, LocalTable,
            LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult, Reverser,
            TranslationUnit};

pub type Restore = Reverser<Drop>;

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
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let local = local(&operands[0])?;
		let table = local_table(&base_function(context, unit, span));
		Ok(Box::new(error(Drop::new(table?, local), span)?))
	}
}

impl Operation for Drop {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let local = context.frame()?.table()[&self.local].clone();
		local.drop(context.drop_stack());
		Ok(())
	}

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
