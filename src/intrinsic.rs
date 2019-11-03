use crate::declaration::{DeclarationPath, ModulePath, StructurePath};

#[derive(Debug)]
pub enum Intrinsic {
	Unsigned8,
	Unsigned16,
	Unsigned32,
	Unsigned64,
	Signed8,
	Signed16,
	Signed32,
	Signed64,
	Truth,
	Tuple,
	Unit,
}

impl Intrinsic {
	pub fn structure(&self) -> StructurePath {
		StructurePath(DeclarationPath {
			module_path: ModulePath::intrinsic(),
			identifier: self.to_string().into(),
		})
	}

	pub fn parse(string: &str) -> Option<Self> {
		Some(match string {
			"u8" => Intrinsic::Unsigned8,
			"u16" => Intrinsic::Unsigned16,
			"u32" => Intrinsic::Unsigned32,
			"u64" => Intrinsic::Unsigned64,
			"i8" => Intrinsic::Signed8,
			"i16" => Intrinsic::Signed16,
			"i32" => Intrinsic::Signed32,
			"i64" => Intrinsic::Signed64,
			"truth" => Intrinsic::Truth,
			_ => return None,
		})
	}

	pub fn to_string(&self) -> &'static str {
		match self {
			Intrinsic::Unsigned8 => "u8",
			Intrinsic::Unsigned16 => "u16",
			Intrinsic::Unsigned32 => "u32",
			Intrinsic::Unsigned64 => "u64",
			Intrinsic::Signed8 => "i8",
			Intrinsic::Signed16 => "i16",
			Intrinsic::Signed32 => "i32",
			Intrinsic::Signed64 => "i64",
			Intrinsic::Truth => "truth",
			Intrinsic::Tuple => "tuple",
			Intrinsic::Unit => "unit",
		}
	}
}
