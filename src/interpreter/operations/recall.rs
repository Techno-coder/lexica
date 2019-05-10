use std::fmt;

use crate::source::Span;

use super::{Call, CompilationUnit, CompileContext, CompileError, CompileResult, Context, Direction,
            FunctionOffset, GenericOperation, InstructionTarget, InterpreterResult, Operand,
            Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Recall {
	call: Call,
}

impl Operational for Recall {
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
		Ok(Box::new(Recall { call: Call::new(target, reverse_target, Direction::Reverse) }))
	}
}

impl Operation for Recall {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.call.execute(context, unit)
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Recall {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.call.reverse(context, unit)
	}
}

impl fmt::Display for Recall {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.call)
	}
}
