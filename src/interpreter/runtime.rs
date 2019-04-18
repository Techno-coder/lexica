use super::{CallFrame, CompilationUnit, Context, Direction, Instruction,
            InstructionTarget, InterpreterError, InterpreterResult, Operation};

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

	/// Returns true if the interpreter should exit
	pub fn force_step(&mut self) -> InterpreterResult<bool> {
		let InstructionTarget(index) = self.context.program_counter();
		let instruction = &self.compilation_unit.instructions[index];
		match instruction.operation {
			Operation::Exit => return Ok(true),
			_ => (),
		}

		let (context, unit) = (&mut self.context, &self.compilation_unit);
		match instruction.polarization {
			Some(Direction::Advance) | None => match instruction.direction {
				Direction::Advance => instruction.operation.execute(context, unit)?,
				Direction::Reverse => instruction.operation.reverse(context, unit)?,
			},
			_ => (),
		}

		let next_instruction = InstructionTarget(index + 1);
		self.context.set_next_instruction(next_instruction);
		self.context.advance()?;
		Ok(false)
	}

	pub fn context(&self) -> &Context {
		&self.context
	}

	pub fn current_instruction(&self) -> &Instruction {
		let InstructionTarget(index) = self.context.program_counter();
		&self.compilation_unit.instructions[index]
	}
}
