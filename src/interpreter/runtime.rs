use super::{CallFrame, CompilationUnit, Context, Direction, Instruction, InstructionTarget,
            InterpreterError, InterpreterResult, Operation, Step};

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
				Step::Exit => break Ok(()),
				_ => (),
			}
		}
	}

	pub fn step(&mut self, direction: Direction) -> InterpreterResult<Step> {
		loop {
			let step = match direction {
				Direction::Advance => self.force_advance(),
				Direction::Reverse => self.force_reverse(),
			}?;

			match step {
				Step::Pass => (),
				_ => break Ok(step),
			}
		}
	}

	pub fn force_advance(&mut self) -> InterpreterResult<Step> {
		let InstructionTarget(index) = self.context.program_counter();
		let instruction = &self.compilation_unit.instructions[index];

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) | None => {
				match instruction.operation {
					Operation::Exit => {
						self.advance_instruction(Direction::Reverse)?;
						return Ok(Step::Exit);
					}
					Operation::ReversalHint => {
						self.advance_instruction(Direction::Advance)?;
						return Ok(Step::ReversalHint);
					}
					_ => match instruction.direction {
						Direction::Advance => instruction.operation.execute(context, unit)?,
						Direction::Reverse => instruction.operation.reverse(context, unit)?,
					}
				}
			}
			_ => (),
		}

		self.advance_instruction(Direction::Advance)?;
		Ok(Step::Pass)
	}

	pub fn force_reverse(&mut self) -> InterpreterResult<Step> {
		let InstructionTarget(index) = self.context.program_counter();
		let instruction = &self.compilation_unit.instructions[index];

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) => (),
			_ => match instruction.operation {
				Operation::Exit => {
					self.advance_instruction(Direction::Advance)?;
					return Ok(Step::Exit);
				}
				Operation::ReversalHint => {
					self.advance_instruction(Direction::Reverse)?;
					return Ok(Step::ReversalHint);
				}
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

		self.advance_instruction(Direction::Reverse)?;
		Ok(Step::Pass)
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
