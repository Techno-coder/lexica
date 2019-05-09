use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterError, InterpreterResult, LocalTable,
            LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult, Primitive,
            Reverser, Reversible, TranslationUnit};

pub type MinusImmediate = Reverser<AddImmediate>;

#[derive(Debug)]
pub struct AddImmediate {
	accumulator: LocalTarget,
	immediate: Primitive,
}

impl AddImmediate {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, immediate: Primitive)
	           -> InterpreterResult<AddImmediate> {
		let accumulator_local = table.local(&accumulator)?;
		match accumulator_local {
			Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
			Primitive::Integer(_) => match immediate {
				Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
				Primitive::Float(_) => Err(InterpreterError::FloatingCast),
				_ => Ok(())
			}
			Primitive::Float(_) => match immediate {
				Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
				_ => Ok(())
			}
		}?;
		Ok(AddImmediate { accumulator, immediate })
	}
}

impl Operational for AddImmediate {
	fn compile<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	               unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, unit, span));
		let (local, primitive) = (local(&operands[0])?, primitive(&operands[1])?);
		Ok(Box::new(error(AddImmediate::new(table?, local, primitive), span)?))
	}
}

impl Operation for AddImmediate {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let accumulator = table.local_mut(&self.accumulator)?;
		match accumulator {
			Primitive::Integer(integer) => match &self.immediate {
				Primitive::Integer(other) => Ok(integer.add(other)),
				_ => Err(InterpreterError::InvalidRuntime)
			}
			Primitive::Float(float) => match &self.immediate {
				Primitive::Integer(other) => Ok(float.add_integer(other)),
				Primitive::Float(other) => Ok(float.add(other)),
				_ => Err(InterpreterError::InvalidRuntime)
			}
			_ => Err(InterpreterError::InvalidRuntime),
		}
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for AddImmediate {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let accumulator = table.local_mut(&self.accumulator)?;
		match accumulator {
			Primitive::Integer(integer) => match &self.immediate {
				Primitive::Integer(other) => Ok(integer.minus(other)),
				_ => Err(InterpreterError::InvalidRuntime)
			}
			Primitive::Float(float) => match &self.immediate {
				Primitive::Integer(other) => Ok(float.minus_integer(other)),
				Primitive::Float(other) => Ok(float.minus(other)),
				_ => Err(InterpreterError::InvalidRuntime)
			}
			_ => Err(InterpreterError::InvalidRuntime),
		}
	}
}

impl fmt::Display for AddImmediate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.accumulator, self.immediate)
	}
}
