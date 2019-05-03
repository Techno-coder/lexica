use std::fmt;

use super::{CompilationUnit, Context, Direction, InstructionTarget, InterpreterResult, Operation};

#[derive(Debug)]
pub struct Return;

impl Operation for Return {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame = context.frame()?;
		let target = frame.return_target().clone();
		let InstructionTarget(return_target) = target;

		let next_instruction = match frame.direction() {
			&Direction::Advance => InstructionTarget(return_target + 1),
			&Direction::Reverse => InstructionTarget(return_target - 1),
		};

		context.set_next_instruction(|| Ok(next_instruction));
		context.pop_frame()?;
		Ok(())
	}

	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame = context.frame()?;
		let target = frame.return_target().clone();
		let InstructionTarget(return_target) = target;

		let next_instruction = match frame.direction() {
			&Direction::Advance => InstructionTarget(return_target - 1),
			&Direction::Reverse => InstructionTarget(return_target + 1),
		};

		context.set_next_instruction(|| Ok(next_instruction));
		context.pop_frame()?;
		Ok(())
	}
}

impl fmt::Display for Return {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Ok(())
	}
}
