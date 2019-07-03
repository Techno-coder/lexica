use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation, InterpreterError,
            InterpreterResult, Operand, Operation, Operational, Primitive, Reversible, Size};

#[derive(Debug)]
pub struct DropImmediate {
	immediate: Primitive,
}

impl DropImmediate {
	pub fn new(size: Size, immediate: Primitive) -> InterpreterResult<DropImmediate> {
		match immediate.cast(size) {
			Some(immediate) => Ok(DropImmediate { immediate }),
			None => Err(InterpreterError::TypesIncompatible),
		}
	}
}

impl Operational for DropImmediate {
	fn arity() -> usize { 2 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], _: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let (size, primitive) = (size(&operands[0])?, primitive(&operands[1])?);
		Ok(Box::new(error(DropImmediate::new(size, primitive), span)?))
	}
}

impl Operation for DropImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		Ok(self.immediate.clone().drop(context.drop_stack()))
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for DropImmediate {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let byte_count = self.immediate.size().byte_count();
		for _ in 0..byte_count {
			context.drop_stack().pop_byte()?;
		}
		Ok(())
	}
}

impl fmt::Display for DropImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.immediate)
	}
}
