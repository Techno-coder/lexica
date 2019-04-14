use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive};

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

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
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

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
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
