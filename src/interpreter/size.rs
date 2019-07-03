use std::fmt;

use super::{ParserError, Primitive};

/// Represents possible data types in the interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
	Boolean,
	Unsigned8,
	Unsigned16,
	Unsigned32,
	Unsigned64,
	Signed8,
	Signed16,
	Signed32,
	Signed64,
	Float32,
	Float64,
	Box,
}

impl Size {
	/// Parses a `Size` from a string.
	///
	/// # Errors
	///
	/// If the string does not match a size then an error is returned.
	pub fn parse(string: &str) -> Result<Size, ParserError> {
		Ok(match string {
			"bool" => Size::Boolean,
			"u8" => Size::Unsigned8,
			"u16" => Size::Unsigned16,
			"u32" => Size::Unsigned32,
			"u64" => Size::Unsigned64,
			"i8" => Size::Signed8,
			"i16" => Size::Signed16,
			"i32" => Size::Signed32,
			"i64" => Size::Signed64,
			"f32" => Size::Float32,
			"f64" => Size::Float64,
			"box" => Size::Box,
			_ => return Err(ParserError::InvalidSize(string))
		})
	}

	pub fn to_string(&self) -> &'static str {
		match self {
			Size::Boolean => "bool",
			Size::Unsigned8 => "u8",
			Size::Unsigned16 => "u16",
			Size::Unsigned32 => "u32",
			Size::Unsigned64 => "u64",
			Size::Signed8 => "i8",
			Size::Signed16 => "i16",
			Size::Signed32 => "i32",
			Size::Signed64 => "i64",
			Size::Float32 => "f32",
			Size::Float64 => "f64",
			Size::Box => "box",
		}
	}

	/// Returns the number of bytes this size takes.
	pub fn byte_count(&self) -> usize {
		match self {
			Size::Boolean => 1,
			Size::Unsigned8 => 1,
			Size::Unsigned16 => 2,
			Size::Unsigned32 => 4,
			Size::Unsigned64 => 8,
			Size::Signed8 => 1,
			Size::Signed16 => 2,
			Size::Signed32 => 4,
			Size::Signed64 => 8,
			Size::Float32 => 4,
			Size::Float64 => 8,
			Size::Box => 2, // TODO, Confirm size of box
		}
	}

	/// Creates a zero primitive of this size.
	pub fn primitive(&self) -> Primitive {
		use super::{Integer, Float};
		let target = self.clone();
		match self {
			Size::Boolean => Primitive::Boolean(false),
			Size::Unsigned8 | Size::Unsigned16 | Size::Unsigned32 | Size::Unsigned64 => {
				let integer = Integer::new_unsigned(0).cast(target);
				Primitive::Integer(integer.unwrap())
			}
			Size::Signed8 | Size::Signed16 | Size::Signed32 | Size::Signed64 => {
				let integer = Integer::new_signed(0).cast(target);
				Primitive::Integer(integer.unwrap())
			}
			Size::Float32 => Primitive::Float(Float::Float32(0.0)),
			Size::Float64 => Primitive::Float(Float::Float64(0.0)),
			Size::Box => unimplemented!(),
		}
	}
}

impl fmt::Display for Size {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}
