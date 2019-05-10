use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Direction, FunctionOffset,
            GenericOperation, InstructionTarget, InterpreterResult, Operand, Operation, Operational,
            Reversible};

#[derive(Debug)]
pub struct Return;

impl Operational for Return {
	fn arity() -> usize { 0 }

	fn compile<'a, 'b>(_: &Span, _: &Vec<Operand<'a>>, _: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		Ok(Box::new(Return))
	}
}

impl Operation for Return {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let frame = context.frame()?;
		let target = frame.return_target().clone();
		let InstructionTarget(function, FunctionOffset(return_offset)) = target;

		let next_instruction = match frame.direction() {
			&Direction::Advance => InstructionTarget(function, FunctionOffset(return_offset + 1)),
			&Direction::Reverse => InstructionTarget(function, FunctionOffset(return_offset - 1)),
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
		let InstructionTarget(function, FunctionOffset(return_offset)) = target;

		let next_instruction = match frame.direction() {
			&Direction::Advance => InstructionTarget(function, FunctionOffset(return_offset - 1)),
			&Direction::Reverse => InstructionTarget(function, FunctionOffset(return_offset + 1)),
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
