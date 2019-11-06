use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

use crate::extension::Indent;
use crate::intrinsic::Intrinsic;

#[derive(Clone, PartialEq)]
pub enum Item {
	Truth(bool),
	Signed8(i8),
	Signed16(i16),
	Signed32(i32),
	Signed64(i64),
	Unsigned8(u8),
	Unsigned16(u16),
	Unsigned32(u32),
	Unsigned64(u64),
	Instance(Instance),
	Uninitialised,
	Unit,
}

impl Item {
	pub fn integer(intrinsic: Intrinsic, integer: i128) -> Option<Self> {
		Some(match intrinsic {
			Intrinsic::Signed8 => Item::Signed8(integer as i8),
			Intrinsic::Signed16 => Item::Signed16(integer as i16),
			Intrinsic::Signed32 => Item::Signed32(integer as i32),
			Intrinsic::Signed64 => Item::Signed64(integer as i64),
			Intrinsic::Unsigned8 => Item::Unsigned8(integer as u8),
			Intrinsic::Unsigned16 => Item::Unsigned16(integer as u16),
			Intrinsic::Unsigned32 => Item::Unsigned32(integer as u32),
			Intrinsic::Unsigned64 => Item::Unsigned64(integer as u64),
			_ => return None,
		})
	}
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Truth(truth) => write!(f, "{}", truth),
			Item::Signed8(integer) => write!(f, "{}", integer),
			Item::Signed16(integer) => write!(f, "{}", integer),
			Item::Signed32(integer) => write!(f, "{}", integer),
			Item::Signed64(integer) => write!(f, "{}", integer),
			Item::Unsigned8(integer) => write!(f, "{}", integer),
			Item::Unsigned16(integer) => write!(f, "{}", integer),
			Item::Unsigned32(integer) => write!(f, "{}", integer),
			Item::Unsigned64(integer) => write!(f, "{}", integer),
			Item::Instance(instance) => write!(f, "{}", instance),
			Item::Uninitialised => write!(f, "<!>"),
			Item::Unit => write!(f, "()"),
		}
	}
}

impl fmt::Debug for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Truth(truth) => write!(f, "Truth({})", truth),
			Item::Signed8(integer) => write!(f, "Signed8({})", integer),
			Item::Signed16(integer) => write!(f, "Signed16({})", integer),
			Item::Signed32(integer) => write!(f, "Signed32({})", integer),
			Item::Signed64(integer) => write!(f, "Signed64({})", integer),
			Item::Unsigned8(integer) => write!(f, "Unsigned8({})", integer),
			Item::Unsigned16(integer) => write!(f, "Unsigned16({})", integer),
			Item::Unsigned32(integer) => write!(f, "Unsigned32({})", integer),
			Item::Unsigned64(integer) => write!(f, "Unsigned64({})", integer),
			Item::Instance(instance) => write!(f, "Instance({})", instance),
			Item::Uninitialised => write!(f, "Uninitialised"),
			Item::Unit => write!(f, "Unit"),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instance {
	pub fields: HashMap<Arc<str>, Item>,
}

impl fmt::Display for Instance {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f)?;
		let indent = &mut Indent::new(f);
		for (index, (field, item)) in self.fields.iter().enumerate() {
			match item {
				Item::Instance(_) => write!(indent, "{}: {}", field, item),
				_ => write!(indent, "{}: {},", field, item),
			}?;

			if index < self.fields.len() - 1 { writeln!(indent)?; }
		}
		Ok(())
	}
}
