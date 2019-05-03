use std::fmt;

use super::{CompilationUnit, Context, Direction, InstructionTarget, InterpreterResult, Operation};

#[derive(Debug)]
pub struct Exit;

impl Operation for Exit {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame_direction = context.frame()?.direction().clone();
		match frame_direction {
			Direction::Advance => context.set_is_halted(true),
			Direction::Reverse if context.is_halted() => {
				let InstructionTarget(program_counter) = context.program_counter();
				context.set_next_instruction(|| Ok(InstructionTarget(program_counter - 1)));
				context.set_is_halted(false);
			}
			_ => (),
		};
		Ok(())
	}

	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame_direction = context.frame()?.direction().clone();
		match frame_direction {
			Direction::Reverse => context.set_is_halted(true),
			Direction::Advance if context.is_halted() => {
				let InstructionTarget(program_counter) = context.program_counter();
				context.set_next_instruction(|| Ok(InstructionTarget(program_counter - 1)));
				context.set_is_halted(false);
			}
			_ => (),
		}
		Ok(())
	}
}

impl fmt::Display for Exit {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Ok(())
	}
}
