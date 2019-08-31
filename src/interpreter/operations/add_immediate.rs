use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Float, GenericOperation,
            InterpreterError, InterpreterResult, LocalTable, LocalTarget, Operand, Operation, Operational,
            Primitive, Reverser, Reversible};

pub type MinusImmediate = Reverser<AddImmediate>;

/// Adds an immediate value to the local.
#[derive(Debug)]
pub struct AddImmediate {
	accumulator: LocalTarget,
	immediate: Primitive,
}

impl AddImmediate {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, immediate: Primitive)
	           -> InterpreterResult<AddImmediate> {
		let accumulator_local = table.local(&accumulator)?;
		match (accumulator_local, &immediate) {
			(Primitive::Boolean(_), _) => Err(InterpreterError::NonNumerical),
			(_, Primitive::Boolean(_)) => Err(InterpreterError::NonNumerical),
			(Primitive::Integer(_), Primitive::Float(_)) => Err(InterpreterError::FloatingCast),
			_ => Ok(AddImmediate { accumulator, immediate })
		}
	}
}

impl Operational for AddImmediate {
	fn arity() -> usize { 2 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, span));
		let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
		Ok(Box::new(error(AddImmediate::new(table?, local, primitive), span)?))
	}
}

impl Operation for AddImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let accumulator = table.local_mut(&self.accumulator)?;
		Ok(match (accumulator, &self.immediate) {
			(Primitive::Integer(integer), Primitive::Integer(other)) =>
				integer.add(other),
			(Primitive::Float(float), Primitive::Integer(other)) =>
				float.add(&Float::Float64(other.cast_float())),
			(Primitive::Float(float), Primitive::Float(other)) =>
				float.add(other),
			_ => return Err(InterpreterError::InvalidRuntime)
		})
	}

	fn reversible(&self) -> Option<&dyn Reversible> {
		Some(self)
	}
}

impl Reversible for AddImmediate {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let accumulator = table.local_mut(&self.accumulator)?;
		Ok(match (accumulator, &self.immediate) {
			(Primitive::Integer(integer), Primitive::Integer(other)) =>
				integer.minus(other),
			(Primitive::Float(float), Primitive::Integer(other)) =>
				float.minus(&Float::Float64(other.cast_float())),
			(Primitive::Float(float), Primitive::Float(other)) =>
				float.minus(other),
			_ => return Err(InterpreterError::InvalidRuntime)
		})
	}
}

impl fmt::Display for AddImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.accumulator, self.immediate)
	}
}
