use std::fmt;

use crate::source::Span;

use super::{CallFrame, CompilationUnit, CompileContext, CompileError, CompileResult, Context,
            Direction, FunctionOffset, GenericOperation, InstructionTarget, InterpreterResult,
            Operand, Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Call {
	target: InstructionTarget,
	reverse_target: InstructionTarget,
	direction: Direction,
}

impl Call {
	pub fn new(target: InstructionTarget, reverse_target: InstructionTarget, direction: Direction) -> Call {
		Call { target, reverse_target, direction }
	}
}

impl Operational for Call {
	fn compile<'a, 'b>(_: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let target = target(&operands[0])?;
		let function_target = context.metadata.function_targets.get(&target)
			.ok_or(operands[0].map(|_| CompileError::UndefinedFunction(target.clone())))?;
		let function = context.unit.functions.get(&target).unwrap();

		let target = InstructionTarget(function_target.clone(), FunctionOffset(0));
		let final_instruction = FunctionOffset(function.instructions.len() - 1);
		let reverse_target = InstructionTarget(function_target.clone(), final_instruction);
		Ok(Box::new(Call::new(target, reverse_target, Direction::Advance)))
	}
}

impl Operation for Call {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let InstructionTarget(function_target, _) = self.target.clone();
		let function = unit.function(function_target).expect("Function does not exist");
		context.push_frame(CallFrame::construct(&function, self.direction, context.program_counter()));
		context.set_next_instruction(|| Ok(self.target.clone()));
		Ok(())
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Call {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		let InstructionTarget(function_target, _) = self.target.clone();
		let function = unit.function(function_target).expect("Function does not exist");
		context.push_frame(CallFrame::construct(&function, self.direction, context.program_counter()));
		context.set_next_instruction(|| Ok(self.reverse_target.clone()));
		Ok(())
	}
}

impl fmt::Display for Call {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.target)
	}
}
