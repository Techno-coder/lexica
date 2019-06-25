use super::{CallFrame, CompilationUnit, Context, Direction, FunctionOffset, FunctionTarget,
            Instruction, InstructionTarget, InterpreterError, InterpreterResult, RuntimeStep};

#[derive(Debug)]
pub struct Runtime {
	compilation_unit: CompilationUnit,
	context: Context,
}

impl Runtime {
	pub fn new(compilation_unit: CompilationUnit) -> InterpreterResult<Runtime> {
		let function_index = compilation_unit.main.clone()
			.ok_or(InterpreterError::MissingEntryPoint)?;
		let entry_function = &compilation_unit.function(function_index.clone()).unwrap();

		let mut context = Context::new(InstructionTarget(function_index, FunctionOffset(0)));
		let null_target = InstructionTarget(FunctionTarget(0), FunctionOffset(0));
		context.push_frame(CallFrame::construct(entry_function, Direction::Advance, null_target));
		Ok(Runtime { compilation_unit, context })
	}

	pub fn run(&mut self, direction: Direction) -> InterpreterResult<()> {
		loop {
			match self.step(direction)? {
				RuntimeStep::Halted => break Ok(()),
				_ => (),
			}
		}
	}

	/// Forcibly steps the runtime until a trap or halt is encountered.
	pub fn step(&mut self, direction: Direction) -> InterpreterResult<RuntimeStep> {
		loop {
			let step = self.force_step(direction)?;
			if step.pauses() {
				break Ok(step);
			}
		}
	}

	/// Forces the runtime to step depending on the runtime and frame direction.
	pub fn force_step(&mut self, direction: Direction) -> InterpreterResult<RuntimeStep> {
		let frame_direction = self.context.frame()?.direction();
		let step = match Direction::compose(frame_direction, direction) {
			Direction::Advance => self.force_advance(),
			Direction::Reverse => self.force_reverse(),
		}?;

		if self.context.is_halted {
			return Ok(RuntimeStep::Halted);
		}

		let frame_direction = self.context.frame()?.direction();
		let composition = Direction::compose(frame_direction, direction);
		self.advance_instruction(composition)?;

		match self.context.is_trapped {
			true => Ok(RuntimeStep::Trapped),
			false => Ok(step),
		}
	}

	/// Forces the runtime to advance regardless of frame direction.
	pub fn force_advance(&mut self) -> InterpreterResult<RuntimeStep> {
		let target = self.context.program_counter();
		let instruction = self.compilation_unit.instruction(target)
			.ok_or(InterpreterError::InstructionBoundary)?;

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) | None => match instruction.direction {
				Direction::Advance => instruction.operation.execute(context, unit)?,
				Direction::Reverse => match instruction.operation.reversible() {
					Some(reversible) => reversible.reverse(context, unit)?,
					None => return Err(InterpreterError::Irreversible)
				}
			}
			_ => (),
		}
		Ok(RuntimeStep::Pass)
	}

	/// Forces the runtime to reverse regardless of frame direction.
	pub fn force_reverse(&mut self) -> InterpreterResult<RuntimeStep> {
		let target = self.context.program_counter();
		let instruction = self.compilation_unit.instruction(target)
			.ok_or(InterpreterError::InstructionBoundary)?;

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Reverse) => match instruction.direction {
				Direction::Advance => instruction.operation.execute(context, unit)?,
				Direction::Reverse => match instruction.operation.reversible() {
					Some(reversible) => reversible.reverse(context, unit)?,
					None => return Err(InterpreterError::Irreversible)
				}
			},
			None => match instruction.direction {
				Direction::Advance => match instruction.operation.reversible() {
					Some(reversible) => reversible.reverse(context, unit)?,
					None => return Err(InterpreterError::Irreversible)
				}
				Direction::Reverse => instruction.operation.execute(context, unit)?,
			},
			_ => (),
		}
		Ok(RuntimeStep::Pass)
	}

	pub fn advance_instruction(&mut self, direction: Direction) -> InterpreterResult<()> {
		let InstructionTarget(target, FunctionOffset(offset)) = self.context.program_counter();
		self.context.set_next_instruction(|| match direction {
			Direction::Advance => Ok(InstructionTarget(target, FunctionOffset(offset + 1))),
			Direction::Reverse => Ok(InstructionTarget(target, FunctionOffset(offset.checked_sub(1)
				.ok_or(InterpreterError::InstructionBoundary)?))),
		});
		self.context.advance()?;
		Ok(())
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn current_instruction(&self) -> InterpreterResult<&Instruction> {
		let target = self.context.program_counter();
		self.compilation_unit.instruction(target)
			.ok_or(InterpreterError::InstructionBoundary)
	}
}
