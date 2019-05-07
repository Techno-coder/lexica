use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterError, InterpreterResult, LocalTable,
            LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult, Primitive,
            TranslationUnit};

#[derive(Debug)]
pub struct Reset {
	local: LocalTarget,
	immediate: Primitive,
}

impl Reset {
	pub fn new(table: &LocalTable, local: LocalTarget, immediate: Primitive)
	           -> InterpreterResult<Reset> {
		let table_local = table.local(&local)?;
		match immediate.cast(table_local.size()) {
			Some(immediate) => Ok(Reset { local, immediate }),
			None => Err(InterpreterError::TypesIncompatible),
		}
	}
}

impl Operational for Reset {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, unit, span));
		let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
		Ok(Box::new(error(Reset::new(table?, local, primitive), span)?))
	}
}

impl Operation for Reset {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		use std::mem;
		let local = &mut context.frame()?.table_mut()[&self.local];
		mem::replace(local, self.immediate.clone());
		Ok(())
	}
}

impl fmt::Display for Reset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.local, self.immediate)
	}
}
