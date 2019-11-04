use std::fmt::{self, Write};
use std::collections::HashMap;
use std::sync::Arc;
use crate::extension::Indent;

#[derive(Clone, PartialEq)]
pub enum Item {
	Truth(bool),
	Signed64(i64),
	Unsigned64(u64),
	Instance(Instance),
	Uninitialised,
	Unit,
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Truth(truth) => write!(f, "{}", truth),
			Item::Signed64(integer) => write!(f, "{}", integer),
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
			Item::Signed64(integer) => write!(f, "Signed64({})", integer),
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
