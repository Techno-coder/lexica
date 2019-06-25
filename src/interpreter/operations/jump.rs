use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, FunctionOffset, GenericOperation,
            InstructionTarget, InterpreterResult, Operand, Operation, Operational};

#[derive(Debug)]
pub struct Jump {
	target: FunctionOffset,
}

impl Jump {
	pub fn new(target: FunctionOffset) -> Jump {
		Jump { target }
	}
}

impl Operational for Jump {
	fn arity() -> usize { 1 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let function = base_function(context, span)?;
		let target = target_label(span, &operands[0], function)?;
		Ok(Box::new(Jump::new(target)))
	}
}

impl Operation for Jump {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let InstructionTarget(function, _) = context.program_counter();
		let next_target = InstructionTarget(function, self.target.clone());
		context.set_next_instruction(|| Ok(next_target));
		Ok(())
	}
}

impl fmt::Display for Jump {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
