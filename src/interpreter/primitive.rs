use super::{DropStack, Float, Integer, InterpreterResult, Size};

pub type Endian = byteorder::LittleEndian;

#[derive(Debug, Clone)]
pub enum Primitive {
	Boolean(bool),
	Integer(Integer),
	Float(Float),
}

impl Primitive {
	pub fn size(&self) -> Size {
		match self {
			Primitive::Boolean(_) => Size::Boolean,
			Primitive::Integer(integer) => integer.size(),
			Primitive::Float(float) => float.size(),
		}
	}

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
}
