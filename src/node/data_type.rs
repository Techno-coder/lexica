use polytype::Type;

use super::Identifier;

/// Sentinel value checks all data types have been visited.
pub const TYPE_SENTINEL: Type<Identifier> = Type::Variable(0);
pub const BOOLEAN_TYPE: Type<Identifier> = Type::Constructed(Identifier("bool"), Vec::new());

const INTRINSICS: &[DataType] = &[DataType::UNIT, DataType::EMPTY];

#[derive(Debug, Clone, PartialEq)]
pub struct DataType<'a>(pub Type<Identifier<'a>>);

impl<'a> DataType<'a> {
	/// Data type representing an zero sized value.
	pub const UNIT: DataType<'static> = DataType::new(Identifier("()"));
	/// Data type representing a value that cannot be constructed.
	pub const EMPTY: DataType<'static> = DataType::new(Identifier("!"));
	pub const BOOLEAN: DataType<'static> = DataType(BOOLEAN_TYPE);

	pub const fn new(identifier: Identifier<'a>) -> Self {
		DataType(Type::Constructed(identifier, Vec::new()))
	}

	pub fn resolved(&self) -> Option<&'a str> {
		match self {
			DataType(Type::Constructed(Identifier(string), _)) => Some(string),
			DataType(Type::Variable(_)) => None,
		}
	}

	/// Returns whether the data type is an intrinsic type.
	pub fn is_intrinsic(&self) -> bool {
		use crate::interpreter::Size;
		match self.resolved() {
			Some(identifier) => Size::parse(identifier).is_ok() || INTRINSICS.contains(self),
			None => false,
		}
	}
}

impl From<crate::interpreter::Size> for DataType<'static> {
	fn from(size: crate::interpreter::Size) -> Self {
		DataType::new(Identifier(size.to_string()))
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
