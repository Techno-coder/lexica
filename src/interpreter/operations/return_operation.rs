use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, Direction, GenericOperation, InstructionTarget, InterpreterResult,
            Operand, Operation, Operational, ParserContext, ParserResult, Reversible, TranslationUnit};

#[derive(Debug)]
pub struct Return;

impl Operational for Return {
	fn parse<'a>(_: &Span, _: &Vec<Operand<'a>>, _: &ParserContext,
	             _: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		Ok(Box::new(Return))
	}
}

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

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Return {
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
	fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
		Ok(())
	}
}
