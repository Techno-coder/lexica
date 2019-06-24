use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier<'a>(pub &'a str);

impl<'a> fmt::Display for Identifier<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let Identifier(string) = self;
		write!(f, "{}", string)
	}
}

#[derive(Debug, PartialEq)]
pub struct DataType<'a>(pub Identifier<'a>);

impl<'a> fmt::Display for DataType<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let DataType(identifier) = self;
		write!(f, "{}", identifier)
	}
}

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
	pub identifier: Identifier<'a>,
	pub data_type: Option<DataType<'a>>,
	pub is_mutable: bool,
}

impl<'a> fmt::Display for Variable<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let prefix = if self.is_mutable { "~" } else { "" };
		if let Some(data_type) = &self.data_type {
			write!(f, "{}{}: {}", prefix, self.identifier, data_type)
		} else {
			write!(f, "{}{}", prefix, self.identifier)
		}
	}
}
