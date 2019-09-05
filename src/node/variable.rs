use std::fmt;

use crate::source::Spanned;

use super::{AccessorTarget, DataType, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable<'a> {
	pub target: VariableTarget<'a>,
	pub data_type: DataType<'a>,
	pub is_mutable: bool,
}

impl<'a> fmt::Display for Variable<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let prefix = if self.is_mutable { "~" } else { "" };
		match self.data_type.resolved() {
			Some(data_type) => write!(f, "{}{}: {}", prefix, self.target, data_type),
			_ => write!(f, "{}{}", prefix, self.target),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VariableTarget<'a>(pub Identifier<'a>, pub usize, pub AccessorTarget<'a>);

impl<'a> VariableTarget<'a> {
	pub fn new_root(identifier: Identifier<'a>, generation: usize) -> Self {
		VariableTarget(identifier, generation, AccessorTarget::default())
	}

	pub fn is_root(&self) -> bool {
		let VariableTarget(_, _, AccessorTarget(accessor)) = self;
		accessor.is_empty()
	}

	pub fn root(&self) -> VariableTarget<'a> {
		let VariableTarget(identifier, generation, _) = self;
		Self::new_root(identifier.clone(), *generation)
	}
}

impl<'a> From<Identifier<'a>> for VariableTarget<'a> {
	fn from(identifier: Identifier<'a>) -> Self {
		let Identifier(string) = identifier;
		match string.find('.') {
			Some(index) => {
				let (identifier, accessor) = string.split_at(index);
				let accessor = AccessorTarget(&accessor[1..]);
				VariableTarget(Identifier(identifier), 0, accessor)
			}
			None => Self::new_root(identifier, 0)
		}
	}
}

impl<'a> From<Spanned<Identifier<'a>>> for Spanned<VariableTarget<'a>> {
	fn from(other: Spanned<Identifier<'a>>) -> Self {
		Spanned::new(other.node.into(), other.span)
	}
}

impl<'a> fmt::Display for VariableTarget<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let VariableTarget(identifier, generation, accessor) = self;
		write!(f, "{}", identifier)?;

		let Identifier(identifier) = identifier;
		if identifier.chars().next() == Some(Identifier::TEMPORARY_PREFIX) {
			write!(f, "${}", generation)?;
		}

		if !self.is_root() {
			write!(f, ".{}", accessor)?;
		}
		Ok(())
	}
}
