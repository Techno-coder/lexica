use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InstructionTarget, InterpreterResult, Operand,
            Operation, Operational, ParserContext, ParserResult, TranslationUnit};

#[derive(Debug)]
pub struct Jump {
	target: InstructionTarget,
}

impl Jump {
	pub fn new(target: InstructionTarget) -> Jump {
		Jump { target }
	}
}

impl Operational for Jump {
	fn compile<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	               unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let target = target_label(span, &operands[0], unit, context)?;
		Ok(Box::new(Jump::new(target)))
	}
}

impl Operation for Jump {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		context.set_program_counter(self.target.clone());
		Ok(())
	}
}

impl fmt::Display for Jump {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
