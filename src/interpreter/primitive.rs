use std::fmt;

use super::{DropStack, Float, Integer, InterpreterResult, Size};

pub type Endian = byteorder::LittleEndian;

/// A basic data type in the interpreter storing a value.
#[derive(Debug, Clone)]
pub enum Primitive {
	Boolean(bool),
	Integer(Integer),
	Float(Float),
}

impl Primitive {
	/// Returns size of the primitive in bytes.
	pub fn size(&self) -> Size {
		match self {
			Primitive::Boolean(_) => Size::Boolean,
			Primitive::Integer(integer) => integer.size(),
			Primitive::Float(float) => float.size(),
		}
	}

	/// Pushes the primitive value onto a drop stack.
	pub fn drop(&self, drop_stack: &mut DropStack) {
		match self {
			Primitive::Boolean(boolean) => match boolean {
				false => drop_stack.push_byte(0b0000_0000),
				true => drop_stack.push_byte(0b1111_1111),
			},
			Primitive::Integer(integer) => integer.drop(drop_stack),
			Primitive::Float(float) => float.drop(drop_stack),
		}
	}

	/// Pops a value off the drop stack and stores it into the primitive.
	pub fn restore(&mut self, drop_stack: &mut DropStack) -> InterpreterResult<()> {
		match self {
			Primitive::Boolean(boolean) => {
				*boolean = match drop_stack.pop_byte()? {
					0b0000_0000 => false,
					_ => true,
				};
				Ok(())
			}
			Primitive::Integer(integer) => integer.restore(drop_stack),
			Primitive::Float(float) => float.restore(drop_stack),
		}
	}

	/// Casts the primitive into another primitive of a compatible size.
	///
	/// Returns `None` if the sizes are not compatible.
	pub fn cast(self, target: Size) -> Option<Primitive> {
		Some(match self {
			Primitive::Boolean(_) if target == Size::Boolean => self,
			Primitive::Integer(integer) => Primitive::Integer(integer.cast(target)?),
			Primitive::Float(float) => Primitive::Float(float.cast(target)?),
			_ => return None,
		})
	}
}

impl fmt::Display for Primitive {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Primitive::Boolean(boolean) => write!(f, "{}", boolean),
			Primitive::Integer(integer) => match integer.is_signed() {
				false => write!(f, "{}", integer.extend_unsigned()),
				true => write!(f, "{}", integer.extend_signed()),
			}
			Primitive::Float(float) => write!(f, "{}", float.extend()),
		}
	}
}
