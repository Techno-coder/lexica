use super::{Context, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive};

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

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
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

	pub fn reverse(&self, context: &mut Context) -> InterpreterResult<()> {
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
