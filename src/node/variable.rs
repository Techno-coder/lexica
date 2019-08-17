use std::fmt;

use crate::source::Spanned;

use super::{DataType, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable<'a> {
	pub target: VariableTarget<'a>,
	pub data_type: DataType<'a>,
	pub is_mutable: bool,
}

impl<'a> fmt::Display for Variable<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let prefix = if self.is_mutable { "~" } else { "" };
		if let Some(data_type) = self.data_type.resolved() {
			write!(f, "{}{}: {}", prefix, self.target, data_type)
		} else {
			write!(f, "{}{}", prefix, self.target)
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VariableTarget<'a>(pub Identifier<'a>, pub usize);

impl<'a> From<Identifier<'a>> for VariableTarget<'a> {
	fn from(identifier: Identifier<'a>) -> Self {
		VariableTarget(identifier, 0)
	}
}

impl<'a> From<Spanned<Identifier<'a>>> for Spanned<VariableTarget<'a>> {
	fn from(other: Spanned<Identifier<'a>>) -> Self {
		Spanned::new(other.node.into(), other.span)
	}
}

impl<'a> fmt::Display for VariableTarget<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let VariableTarget(identifier, generation) = self;
		write!(f, "{}", identifier)?;

		if identifier == &Identifier::TEMPORARY {
			write!(f, "{}", generation)?;
		}

		Ok(())
	}
}
