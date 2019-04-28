use super::{Context, InstructionTarget, InterpreterResult, Direction};

#[derive(Debug)]
pub struct Return;

impl Return {
	pub fn execute(context: &mut Context) -> InterpreterResult<()> {
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

	pub fn reverse(context: &mut Context) -> InterpreterResult<()> {
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
