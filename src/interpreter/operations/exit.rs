use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Direction, FunctionOffset,
            GenericOperation, InstructionTarget, InterpreterResult, Operand, Operation, Operational,
            Reversible};

#[derive(Debug)]
pub struct Exit;

impl Operational for Exit {
	fn arity() -> usize { 0 }

	fn compile<'a>(_: &Span, _: &Vec<Operand<'a>>, _: &CompileContext) -> CompileResult<'a, GenericOperation> {
		Ok(Box::new(Exit))
	}
}

impl Operation for Exit {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame_direction = context.frame()?.direction().clone();
		match frame_direction {
			Direction::Advance => context.is_halted = true,
			Direction::Reverse if context.is_halted => {
				let InstructionTarget(function, FunctionOffset(offset)) = context.program_counter();
				context.set_next_instruction(|| Ok(InstructionTarget(function, FunctionOffset(offset - 1))));
				context.is_halted = false;
			}
			_ => (),
		};
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Exit {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame_direction = context.frame()?.direction().clone();
		match frame_direction {
			Direction::Reverse => context.is_halted = true,
			Direction::Advance if context.is_halted => {
				let InstructionTarget(function, FunctionOffset(offset)) = context.program_counter();
				context.set_next_instruction(|| Ok(InstructionTarget(function, FunctionOffset(offset - 1))));
				context.is_halted = false;
			}
			_ => (),
		}
		Ok(())
	}
}

impl fmt::Display for Exit {
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
