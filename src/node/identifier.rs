use std::fmt;

use polytype::{Name, Type};

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

#[derive(Debug, Clone, PartialEq)]
pub struct DataType<'a>(pub Type<Identifier<'a>>);

impl<'a> DataType<'a> {
	pub fn new(identifier: Identifier<'a>) -> Self {
		DataType(Type::Constructed(identifier, Vec::new()))
	}

	pub fn resolved(&self) -> Option<&'a str> {
		match self {
			DataType(Type::Constructed(Identifier(string), _)) => Some(string),
			DataType(Type::Variable(_)) => None,
		}
	}
}

impl<'a> AsRef<Type<Identifier<'a>>> for DataType<'a> {
	fn as_ref(&self) -> &Type<Identifier<'a>> {
		let DataType(data_type) = self;
		data_type
	}
}

impl<'a> AsMut<Type<Identifier<'a>>> for DataType<'a> {
	fn as_mut(&mut self) -> &mut Type<Identifier<'a>> {
		let DataType(data_type) = self;
		data_type
	}
}

impl<'a> Default for DataType<'a> {
	fn default() -> Self {
		DataType(Type::Variable(0))
	}
}
