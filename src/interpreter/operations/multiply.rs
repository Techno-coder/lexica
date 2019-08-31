use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Float, GenericOperation,
            InterpreterError, InterpreterResult, LocalTable, LocalTarget, Operand, Operation, Operational,
            Primitive, Reversible};

/// Multiplies two locals together and stores the result in the first.
#[derive(Debug)]
pub struct Multiply {
	accumulator: LocalTarget,
	operand: LocalTarget,
}

impl Multiply {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, operand: LocalTarget)
	           -> InterpreterResult<Multiply> {
		let operand_local = &table.local(&operand)?;
		let accumulator_local = table.local(&accumulator)?;
		match accumulator_local {
			Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
			Primitive::Integer(_) => match operand_local {
				Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
				Primitive::Float(_) => Err(InterpreterError::FloatingCast),
				_ => Ok(())
			}
			Primitive::Float(_) => match operand_local {
				Primitive::Boolean(_) => Err(InterpreterError::NonNumerical),
				_ => Ok(())
			}
		}?;
		Ok(Multiply { accumulator, operand })
	}
}

impl Operational for Multiply {
	fn arity() -> usize { 2 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, span));
		let (left, right) = (local(&operands[0])?, local(&operands[1])?);
		Ok(Box::new(error(Multiply::new(table?, left, right), span)?))
	}
}

impl Operation for Multiply {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];
		match accumulator {
			Primitive::Integer(integer) => match operand {
				Primitive::Integer(other) => Ok(integer.multiply(other)?),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			Primitive::Float(float) => match operand {
				Primitive::Integer(other) => {
					let other = Float::Float64(other.cast_float());
					Ok(float.multiply(&other)?)
				}
				Primitive::Float(other) => Ok(float.multiply(other)?),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			_ => Err(InterpreterError::InvalidRuntime)
		}
	}

	fn reversible(&self) -> Option<&dyn Reversible> {
		match self.accumulator == self.operand {
			false => Some(self),
			true => None,
		}
	}
}

impl Reversible for Multiply {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];
		match accumulator {
			Primitive::Integer(integer) => match operand {
				Primitive::Integer(other) => Ok(integer.divide(other)?),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			Primitive::Float(float) => match operand {
				Primitive::Integer(other) => {
					let other = Float::Float64(other.cast_float());
					Ok(float.divide(&other)?)
				}
				Primitive::Float(other) => Ok(float.divide(other)?),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			_ => Err(InterpreterError::InvalidRuntime)
		}
	}
}

impl fmt::Display for Multiply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.accumulator, self.operand)
	}
}
