use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use super::{DropStack, Endian, Integer, InterpreterResult, Size};

#[derive(Debug, Clone)]
pub enum Float {
	Float32(f32),
	Float64(f64),
}

impl Float {
	pub fn size(&self) -> Size {
		match self {
			Float::Float32(_) => Size::Float32,
			Float::Float64(_) => Size::Float64,
		}
	}

	pub fn drop(&self, drop_stack: &mut DropStack) {
		let mut bytes = Vec::new();
		match self {
			Float::Float32(float) => bytes.write_f32::<Endian>(*float),
			Float::Float64(float) => bytes.write_f64::<Endian>(*float),
		}.expect("Failed to drop float");
		bytes.into_iter().for_each(|byte| drop_stack.push_byte(byte));
	}

	pub fn restore(&mut self, drop_stack: &mut DropStack) -> InterpreterResult<()> {
		let bytes: Result<Vec<_>, _> = (0..self.size().byte_count())
			.map(|_| drop_stack.pop_byte()).collect();
		let mut bytes = Cursor::new(bytes?);
		(|| -> std::io::Result<()> {
			Ok(match self {
				Float::Float32(float) => *float = bytes.read_f32::<Endian>()?,
				Float::Float64(float) => *float = bytes.read_f64::<Endian>()?,
			})
		})().expect("Failed to read float");
		Ok(())
	}

	pub fn extend(&self) -> f64 {
		match self {
			Float::Float32(float) => *float as f64,
			Float::Float64(float) => *float,
		}
	}

	pub fn add_integer(&mut self, integer: &Integer) {
		match self {
			Float::Float32(float) => match integer.is_signed() {
				false => *float += integer.extend_unsigned() as f32,
				true => *float += integer.extend_signed() as f32,
			},
			Float::Float64(float) => match integer.is_signed() {
				false => *float += integer.extend_unsigned() as f64,
				true => *float += integer.extend_signed() as f64,
			},
		}
	}

	pub fn minus_integer(&mut self, integer: &Integer) {
		match self {
			Float::Float32(float) => match integer.is_signed() {
				false => *float -= integer.extend_unsigned() as f32,
				true => *float -= integer.extend_signed() as f32,
			},
			Float::Float64(float) => match integer.is_signed() {
				false => *float -= integer.extend_unsigned() as f64,
				true => *float -= integer.extend_signed() as f64,
			},
		}
	}

	pub fn add(&mut self, other: &Float) {
		match self {
			Float::Float32(float) => *float += other.extend() as f32,
			Float::Float64(float) => *float += other.extend(),
		}
	}

	pub fn minus(&mut self, other: &Float) {
		match self {
			Float::Float32(float) => *float -= other.extend() as f32,
			Float::Float64(float) => *float -= other.extend(),
		}
	}

	pub fn cast(self, target: Size) -> Option<Float> {
		let mut float = match target {
			Size::Float32 => Float::Float32(0.0),
			Size::Float64 => Float::Float64(0.0),
			_ => return None,
		};
		float.add(&self);
		Some(float)
	}
}
