use std::fmt;

use polytype::Name;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier<'a>(pub &'a str);

impl<'a> Identifier<'a> {
	pub const TEMPORARY_PREFIX: char = '.';
	pub const TEMPORARY_LOWER: Self = Identifier(".");
	pub const TEMPORARY_REVERSE: Self = Identifier(".reverse.");
}

impl<'a> fmt::Display for Identifier<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Identifier(string) = self;
		write!(f, "{}", string)
	}
}

impl<'a> Name for Identifier<'a> {
	fn arrow() -> Self {
		Identifier("->")
	}

	fn show(&self) -> String {
		self.to_string()
	}
}
