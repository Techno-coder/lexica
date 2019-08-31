use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, Float, GenericOperation,
            InterpreterError, InterpreterResult, LocalTable, LocalTarget, Operand, Operation, Operational,
            Primitive, Reverser, Reversible};

pub type Minus = Reverser<Add>;

/// Adds two locals and stores the result in the first.
/// Wraps on overflow.
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

		match (accumulator_local, operand_local) {
			(Primitive::Boolean(_), _) => Err(InterpreterError::NonNumerical),
			(_, Primitive::Boolean(_)) => Err(InterpreterError::NonNumerical),
			(Primitive::Integer(_), Primitive::Float(_)) => Err(InterpreterError::FloatingCast),
			_ => Ok(Add { accumulator, operand })
		}
	}
}

impl Operational for Add {
	fn arity() -> usize { 2 }

	fn compile<'a, 'b>(span: Span, operands: &[Operand<'a>], context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		use super::unit_parsers::*;
		let table = local_table(&base_function(context, span));
		let (left, right) = (local(&operands[0])?, local(&operands[1])?);
		Ok(Box::new(error(Add::new(table?, left, right), span)?))
	}
}

impl Operation for Add {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];
		Ok(match (accumulator, operand) {
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
		match self.accumulator == self.operand {
			false => Some(self),
			true => None,
		}
	}
}

impl Reversible for Add {
	fn reverse(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let table = context.frame()?.table_mut();
		let operand = &table[&self.operand].clone();
		let accumulator = &mut table[&self.accumulator];

		Ok(match (accumulator, operand) {
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

impl fmt::Display for Add {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.accumulator, self.operand)
	}
}
