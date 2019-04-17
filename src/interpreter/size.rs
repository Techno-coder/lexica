use super::ParserError;

#[derive(Debug, PartialEq)]
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
}
