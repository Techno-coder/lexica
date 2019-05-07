use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, Direction, GenericOperation, InstructionTarget, InterpreterResult,
            Operand, Operation, Operational, ParserContext, ParserResult, TranslationUnit};

#[derive(Debug)]
pub struct Exit;

impl Operational for Exit {
	fn parse<'a>(_: &Span, _: &Vec<Operand<'a>>, _: &ParserContext,
	             _: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		Ok(Box::new(Exit))
	}
}

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
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
