use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Direction, FunctionOffset,
	GenericOperation, InstructionTarget, InterpreterResult, Operand, Operation, Operational,
	Reversible};

/// Halts the runtime if the step direction is the same as exit direction.
#[derive(Debug)]
pub struct Exit;

impl Operational for Exit {
	fn arity() -> usize { 0 }

	fn compile<'a>(_: Span, _: &[Operand<'a>], _: &CompileContext) -> CompileResult<'a, GenericOperation> {
		Ok(Box::new(Exit))
	}
}

impl Operation for Exit {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		context.is_halted = true;
		Ok(())
	}

	fn reversible(&self) -> Option<&dyn Reversible> {
		Some(self)
	}
}

impl Reversible for Exit {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let InstructionTarget(function, FunctionOffset(offset)) = context.program_counter();
		let next_instruction = match context.step_direction {
			Direction::Advance => InstructionTarget(function, FunctionOffset(offset + 1)),
			Direction::Reverse => InstructionTarget(function, FunctionOffset(offset - 1)),
		};

		context.set_next_instruction(|| Ok(next_instruction));
		context.is_halted = false;
		Ok(())
	}
}

impl fmt::Display for Exit {
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
