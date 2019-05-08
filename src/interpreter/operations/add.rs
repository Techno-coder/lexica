use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterError, InterpreterResult, LocalTable,
            LocalTarget, Operand, Operation, Operational, ParserContext, ParserResult, Primitive,
            Reverser, Reversible, TranslationUnit};

pub type Minus = Reverser<Add>;

#[derive(Debug)]
pub struct Add {
	accumulator: LocalTarget,
	operand: LocalTarget,
}

impl Add {
	pub fn new(table: &LocalTable, accumulator: LocalTarget, operand: LocalTarget)
	           -> InterpreterResult<Add> {
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
		Ok(Add { accumulator, operand })
	}
}

impl Operational for Add {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, unit, span));
		let (left, right) = (local(&operands[0])?, local(&operands[1])?);
		Ok(Box::new(error(Add::new(table?, left, right), span)?))
	}
}

impl Operation for Add {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];
		match accumulator {
			Primitive::Integer(integer) => match operand {
				Primitive::Integer(other) => Ok(integer.add(other)),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			Primitive::Float(float) => match operand {
				Primitive::Integer(other) => Ok(float.add_integer(other)),
				Primitive::Float(other) => Ok(float.add(other)),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			_ => Err(InterpreterError::InvalidRuntime)
		}
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl Reversible for Add {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];
		match accumulator {
			Primitive::Integer(integer) => match operand {
				Primitive::Integer(other) => Ok(integer.minus(other)),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			Primitive::Float(float) => match operand {
				Primitive::Integer(other) => Ok(float.minus_integer(other)),
				Primitive::Float(other) => Ok(float.minus(other)),
				_ => Err(InterpreterError::InvalidRuntime),
			}
			_ => Err(InterpreterError::InvalidRuntime)
		}
	}
}

impl fmt::Display for Add {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.accumulator, self.operand)
	}
}
