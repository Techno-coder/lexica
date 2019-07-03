use std::fmt;

use crate::interpreter::Direction;
use crate::source::Span;

use super::{CallFrame, CompilationUnit, CompileContext, CompileError, CompileResult, Context,
            FunctionOffset, FunctionTarget, GenericOperation, InstructionTarget, InterpreterResult,
            Operand, Operation, Operational, Reversible};

/// Calls a function.
#[derive(Debug)]
pub struct Call {
	target: FunctionTarget,
}

impl Call {
	pub fn new(target: FunctionTarget) -> Call {
		Call { target }
	}
}

impl Operational for Call {
	fn arity() -> usize { 1 }

	fn compile<'a, 'b>(_: Span, operands: &[Operand<'a>], context: &CompileContext)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let target = target(&operands[0])?;
		let function_target = context.metadata.function_targets.get(&target)
			.ok_or(operands[0].map(|_| CompileError::UndefinedFunction(target.clone())))?;
		Ok(Box::new(Call::new(function_target.clone())))
	}
}

impl Operation for Call {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let function = unit.function(self.target.clone()).expect("Function does not exist");
		let target = InstructionTarget(self.target.clone(), FunctionOffset(0));
		let frame_direction = Direction::compose(context.step_direction, Direction::Advance);
		context.push_frame(CallFrame::construct(&function, frame_direction, context.program_counter()));
		context.set_next_instruction(|| Ok(target));
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Call {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let function = unit.function(self.target.clone()).expect("Function does not exist");
		let target = InstructionTarget(self.target.clone(), FunctionOffset(function.instructions.len() - 1));
		let frame_direction = Direction::compose(context.step_direction, Direction::Reverse);
		context.push_frame(CallFrame::construct(&function, frame_direction, context.program_counter()));
		context.set_next_instruction(|| Ok(target));
		Ok(())
	}
}

impl fmt::Display for Call {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
