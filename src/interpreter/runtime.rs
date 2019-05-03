use super::{CallFrame, CompilationUnit, Context, Direction, Instruction, InstructionTarget,
            InterpreterError, InterpreterResult, RefactorOperation, RuntimeStep};

#[derive(Debug)]
pub struct Runtime {
	compilation_unit: CompilationUnit,
	context: Context,
}

impl Runtime {
	pub fn new(compilation_unit: CompilationUnit) -> InterpreterResult<Runtime> {
		let entry_point = compilation_unit.main.clone()
			.ok_or(InterpreterError::MissingEntryPoint)?;
		let entry_function = compilation_unit.function_labels
			.get(&entry_point).unwrap();

		let mut context = Context::new(entry_point);
		context.push_frame(CallFrame::construct(entry_function, Direction::Advance, InstructionTarget(0)));
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

	/// Forcibly steps the runtime until a reversal hint is encountered.
	pub fn step(&mut self, direction: Direction) -> InterpreterResult<RuntimeStep> {
		loop {
			let step = self.force_step(direction)?;
			match step {
				RuntimeStep::Pass => (),
				_ => break Ok(step),
			}
		}
	}

	/// Forces the runtime to step depending on the runtime and frame direction.
	pub fn force_step(&mut self, direction: Direction) -> InterpreterResult<RuntimeStep> {
		let frame_direction = self.context.frame()?.direction();
		let step = match Direction::compose(frame_direction, &direction) {
			Direction::Advance => self.force_advance(),
			Direction::Reverse => self.force_reverse(),
		}?;

		if self.context.is_halted() {
			return Ok(RuntimeStep::Halted);
		}

		let frame_direction = self.context.frame()?.direction();
		let composition = Direction::compose(frame_direction, &direction);
		self.advance_instruction(composition)?;
		Ok(step)
	}

	/// Forces the runtime to advance regardless of frame direction.
	pub fn force_advance(&mut self) -> InterpreterResult<RuntimeStep> {
		let InstructionTarget(index) = self.context.program_counter();
		let instruction = &self.compilation_unit.instructions[index];

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) | None => {
				match instruction.operation {
					RefactorOperation::ReversalHint => return Ok(RuntimeStep::ReversalHint),
					_ => match instruction.direction {
						Direction::Advance => instruction.operation.execute(context, unit)?,
						Direction::Reverse => instruction.operation.reverse(context, unit)?,
					}
				}
			}
			_ => (),
		}
		Ok(RuntimeStep::Pass)
	}

	/// Forces the runtime to reverse regardless of frame direction.
	pub fn force_reverse(&mut self) -> InterpreterResult<RuntimeStep> {
		let InstructionTarget(index) = self.context.program_counter();
		let instruction = &self.compilation_unit.instructions[index];

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) => (),
			_ => match instruction.operation {
				RefactorOperation::ReversalHint => return Ok(RuntimeStep::ReversalHint),
				_ => (),
			}
		}

		match instruction.polarization {
			Some(Direction::Reverse) => match instruction.direction {
				Direction::Advance => instruction.operation.execute(context, unit)?,
				Direction::Reverse => instruction.operation.reverse(context, unit)?,
			},
			None => match instruction.direction {
				Direction::Advance => instruction.operation.reverse(context, unit)?,
				Direction::Reverse => instruction.operation.execute(context, unit)?,
			},
			_ => (),
		}
		Ok(RuntimeStep::Pass)
	}

	pub fn advance_instruction(&mut self, direction: Direction) -> InterpreterResult<()> {
		let InstructionTarget(index) = self.context.program_counter();
		self.context.set_next_instruction(|| match direction {
			Direction::Advance => Ok(InstructionTarget(index + 1)),
			Direction::Reverse => Ok(InstructionTarget(index.checked_sub(1)
				.ok_or(InterpreterError::InstructionBoundary)?)),
		});
		self.context.advance()?;
		Ok(())
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn current_instruction(&self) -> &Instruction {
		let InstructionTarget(index) = self.context.program_counter();
		&self.compilation_unit.instructions[index]
	}
}
