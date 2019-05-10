use std::fmt;

use crate::source::Span;

use super::{Call, CompilationUnit, CompileContext, CompileError, CompileResult, Context, Direction,
            GenericOperation, InterpreterResult, Operand,
            Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Recall {
	call: Call,
}

impl Operational for Recall {
	fn arity() -> usize { 1 }

	fn compile<'a, 'b>(_: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let target = target(&operands[0])?;
		let function_target = context.metadata.function_targets.get(&target)
			.ok_or(operands[0].map(|_| CompileError::UndefinedFunction(target.clone())))?;
		Ok(Box::new(Call::new(function_target.clone(), Direction::Reverse)))
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
		write!(f, "{}", self.call)
	}
}
