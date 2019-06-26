use std::fmt;
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::{DropStack, Endian, InterpreterResult, Size};

#[derive(Debug, Clone)]
pub struct Integer {
	data: u64,
	size: Size,
}

impl Integer {
	pub fn new_unsigned(integer: u64) -> Integer {
		Integer { data: integer, size: Size::Unsigned64 }
	}

	pub fn new_signed(integer: i64) -> Integer {
		Integer { data: integer as u64, size: Size::Signed64 }
	}

	pub fn size(&self) -> Size {
		self.size.clone()
	}

	pub fn drop(&self, drop_stack: &mut DropStack) {
		let mut bytes = Vec::new();
		match self.size {
			Size::Unsigned8 | Size::Signed8 => bytes.write_u8(self.data as u8),
			Size::Unsigned16 | Size::Signed16 => bytes.write_u16::<Endian>(self.data as u16),
			Size::Unsigned32 | Size::Signed32 => bytes.write_u32::<Endian>(self.data as u32),
			Size::Unsigned64 | Size::Signed64 => bytes.write_u64::<Endian>(self.data as u64),
			_ => unreachable!(),
		}.unwrap();
		bytes.into_iter().for_each(|byte| drop_stack.push_byte(byte));
	}

	pub fn restore(&mut self, drop_stack: &mut DropStack) -> InterpreterResult<()> {
		let mut bytes = vec![0; self.size.byte_count()];
		for index in (0..bytes.len()).rev() {
			bytes[index] = drop_stack.pop_byte()?;
		}

		let mut bytes = Cursor::new(bytes);
		Ok(match self.size {
			Size::Unsigned8 | Size::Signed8 => self.data = bytes.read_u8().unwrap() as u64,
			Size::Unsigned16 | Size::Signed16 => self.data = bytes.read_u16::<Endian>().unwrap() as u64,
			Size::Unsigned32 | Size::Signed32 => self.data = bytes.read_u32::<Endian>().unwrap() as u64,
			Size::Unsigned64 | Size::Signed64 => self.data = bytes.read_u64::<Endian>().unwrap() as u64,
			_ => unreachable!(),
		})
	}

	/// Truncates the integer to the internal size. Returns true if overflow occurs.
	pub fn maintain(&mut self) -> bool {
		let original = self.data;
		match self.size {
			Size::Unsigned8 => self.data = self.data as u8 as u64,
			Size::Unsigned16 => self.data = self.data as u16 as u64,
			Size::Unsigned32 => self.data = self.data as u32 as u64,
			Size::Signed8 => self.data = self.data as i64 as i8 as i64 as u64,
			Size::Signed16 => self.data = self.data as i64 as i16 as i64 as u64,
			Size::Signed32 => self.data = self.data as i64 as i32 as i64 as u64,
			Size::Unsigned64 | Size::Signed64 => (),
			_ => unreachable!(),
		}
		self.data == original
	}

	pub fn extend_unsigned(&self) -> Result<u64, i64> {
		Ok(match self.size {
			Size::Unsigned8 | Size::Unsigned16 => self.data,
			Size::Unsigned32 | Size::Unsigned64 => self.data,
			_ => return Err(self.data as i64),
		})
	}

	pub fn cast_float(&self) -> f64 {
		match self.extend_unsigned() {
			Ok(integer) => integer as f64,
			Err(integer) => integer as f64,
		}
	}

	pub fn add(&mut self, other: &Self) {
		self.data = self.data.wrapping_add(other.data);
		self.maintain();
	}

	pub fn minus(&mut self, other: &Self) {
		self.data = self.data.wrapping_sub(other.data);
		self.maintain();
	}

	// TODO: Return true if overflow occurs and set entropic fence
	pub fn multiply(&mut self, other: &Self) -> bool {
		unimplemented!()
	}

	pub fn cast(mut self, target: Size) -> Option<Self> {
		match target {
			Size::Unsigned8 | Size::Signed8 => (),
			Size::Unsigned16 | Size::Signed16 => (),
			Size::Unsigned32 | Size::Signed32 => (),
			Size::Unsigned64 | Size::Signed64 => (),
			_ => return None,
		}

		self.size = target;
		self.maintain();
		Some(self)
	}
}

impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.extend_unsigned() {
			Ok(integer) => write!(f, "{}", integer),
			Err(integer) => write!(f, "{}", integer),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_add_unsigned() {
		let mut integer = Integer::new_unsigned(1);
		integer.add(&Integer::new_unsigned(2));
		assert_eq!(integer.extend_unsigned(), Ok(3));
	}

	#[test]
	pub fn test_add_signed() {
		let mut integer = Integer::new_signed(1);
		integer.add(&Integer::new_signed(-2));
		assert_eq!(integer.extend_unsigned(), Err(-1));
	}

	#[test]
	pub fn test_add_mixed() {
		let mut integer = Integer::new_signed(1);
		integer.add(&Integer::new_unsigned(2));
		assert_eq!(integer.extend_unsigned(), Err(3));
	}
}
