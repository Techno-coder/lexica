use polytype::Type;

use super::Identifier;

/// Sentinel value checks all data types have been visited.
pub const TYPE_SENTINEL: Type<Identifier> = Type::Variable(0);

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
		DataType(TYPE_SENTINEL)
	}
}
