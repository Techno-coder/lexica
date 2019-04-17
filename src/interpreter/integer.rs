use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::{DropStack, Endian, InterpreterResult, Size};

#[derive(Debug, Clone)]
pub enum Integer {
	Unsigned8(u8),
	Unsigned16(u16),
	Unsigned32(u32),
	Unsigned64(u64),
	Signed8(i8),
	Signed16(i16),
	Signed32(i32),
	Signed64(i64),
}

impl Integer {
	pub fn size(&self) -> Size {
		match self {
			Integer::Unsigned8(_) => Size::Unsigned8,
			Integer::Unsigned16(_) => Size::Unsigned16,
			Integer::Unsigned32(_) => Size::Unsigned32,
			Integer::Unsigned64(_) => Size::Unsigned64,
			Integer::Signed8(_) => Size::Signed8,
			Integer::Signed16(_) => Size::Signed16,
			Integer::Signed32(_) => Size::Signed32,
			Integer::Signed64(_) => Size::Signed64,
		}
	}

	pub fn drop(&self, drop_stack: &mut DropStack) {
		let mut bytes = Vec::new();
		match self {
			Integer::Unsigned8(integer) => bytes.write_u8(*integer),
			Integer::Unsigned16(integer) => bytes.write_u16::<Endian>(*integer),
			Integer::Unsigned32(integer) => bytes.write_u32::<Endian>(*integer),
			Integer::Unsigned64(integer) => bytes.write_u64::<Endian>(*integer),
			Integer::Signed8(integer) => bytes.write_i8(*integer),
			Integer::Signed16(integer) => bytes.write_i16::<Endian>(*integer),
			Integer::Signed32(integer) => bytes.write_i32::<Endian>(*integer),
			Integer::Signed64(integer) => bytes.write_i64::<Endian>(*integer),
		}.expect("Failed to drop integer");
		bytes.into_iter().for_each(|byte| drop_stack.push_byte(byte));
	}

	pub fn restore(&mut self, drop_stack: &mut DropStack) -> InterpreterResult<()> {
		let bytes: Result<Vec<_>, _> = (0..self.size().byte_count())
			.map(|_| drop_stack.pop_byte()).collect();
		let mut bytes = Cursor::new(bytes?);
		(|| -> std::io::Result<()> {
			Ok(match self {
				Integer::Unsigned8(integer) => *integer = bytes.read_u8()?,
				Integer::Unsigned16(integer) => *integer = bytes.read_u16::<Endian>()?,
				Integer::Unsigned32(integer) => *integer = bytes.read_u32::<Endian>()?,
				Integer::Unsigned64(integer) => *integer = bytes.read_u64::<Endian>()?,
				Integer::Signed8(integer) => *integer = bytes.read_i8()?,
				Integer::Signed16(integer) => *integer = bytes.read_i16::<Endian>()?,
				Integer::Signed32(integer) => *integer = bytes.read_i32::<Endian>()?,
				Integer::Signed64(integer) => *integer = bytes.read_i64::<Endian>()?,
			})
		})().expect("Failed to read integer");
		Ok(())
	}

	pub fn is_signed(&self) -> bool {
		match self {
			Integer::Unsigned8(_) => false,
			Integer::Unsigned16(_) => false,
			Integer::Unsigned32(_) => false,
			Integer::Unsigned64(_) => false,
			_ => true,
		}
	}

	pub fn extend_unsigned(&self) -> u64 {
		match self {
			Integer::Unsigned8(integer) => *integer as u64,
			Integer::Unsigned16(integer) => *integer as u64,
			Integer::Unsigned32(integer) => *integer as u64,
			Integer::Unsigned64(integer) => *integer as u64,
			Integer::Signed8(integer) => *integer as u64,
			Integer::Signed16(integer) => *integer as u64,
			Integer::Signed32(integer) => *integer as u64,
			Integer::Signed64(integer) => *integer as u64,
		}
	}

	pub fn extend_signed(&self) -> i64 {
		match self {
			Integer::Unsigned8(integer) => *integer as i64,
			Integer::Unsigned16(integer) => *integer as i64,
			Integer::Unsigned32(integer) => *integer as i64,
			Integer::Unsigned64(integer) => *integer as i64,
			Integer::Signed8(integer) => *integer as i64,
			Integer::Signed16(integer) => *integer as i64,
			Integer::Signed32(integer) => *integer as i64,
			Integer::Signed64(integer) => *integer as i64,
		}
	}

	pub fn add(&mut self, other: &Integer) {
		let extension = other.extend_unsigned();
		match self {
			Integer::Unsigned8(integer) => *integer = integer.wrapping_add(extension as u8),
			Integer::Unsigned16(integer) => *integer = integer.wrapping_add(extension as u16),
			Integer::Unsigned32(integer) => *integer = integer.wrapping_add(extension as u32),
			Integer::Unsigned64(integer) => *integer = integer.wrapping_add(extension as u64),
			Integer::Signed8(integer) => *integer = integer.wrapping_add(extension as i8),
			Integer::Signed16(integer) => *integer = integer.wrapping_add(extension as i16),
			Integer::Signed32(integer) => *integer = integer.wrapping_add(extension as i32),
			Integer::Signed64(integer) => *integer = integer.wrapping_add(extension as i64),
		}
	}

	pub fn minus(&mut self, other: &Integer) {
		let extension = other.extend_unsigned();
		match self {
			Integer::Unsigned8(integer) => *integer = integer.wrapping_sub(extension as u8),
			Integer::Unsigned16(integer) => *integer = integer.wrapping_sub(extension as u16),
			Integer::Unsigned32(integer) => *integer = integer.wrapping_sub(extension as u32),
			Integer::Unsigned64(integer) => *integer = integer.wrapping_sub(extension as u64),
			Integer::Signed8(integer) => *integer = integer.wrapping_sub(extension as i8),
			Integer::Signed16(integer) => *integer = integer.wrapping_sub(extension as i16),
			Integer::Signed32(integer) => *integer = integer.wrapping_sub(extension as i32),
			Integer::Signed64(integer) => *integer = integer.wrapping_sub(extension as i64),
		}
	}

	pub fn cast(self, target: Size) -> Option<Integer> {
		let mut other = match target {
			Size::Unsigned8 => Integer::Unsigned8(0),
			Size::Unsigned16 => Integer::Unsigned16(0),
			Size::Unsigned32 => Integer::Unsigned32(0),
			Size::Unsigned64 => Integer::Unsigned64(0),
			Size::Signed8 => Integer::Signed8(0),
			Size::Signed16 => Integer::Signed16(0),
			Size::Signed32 => Integer::Signed32(0),
			Size::Signed64 => Integer::Signed64(0),
			_ => return None,
		};
		other.add(&self);
		Some(other)
	}
}
