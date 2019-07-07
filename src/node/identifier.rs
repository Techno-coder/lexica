use std::fmt;

use polytype::Name;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier<'a>(pub &'a str);

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
