use std::fmt;

use crate::source::Spanned;
use crate::utility::PrefixSplit;

use super::{DataType, ExpressionNode, FunctionCall, Identifier, NodeConstruct, NodeVisitor};

// TODO: Change to AccessorCall
#[derive(Debug, Clone)]
pub struct Accessor<'a> {
	pub expression: Spanned<ExpressionNode<'a>>,
	pub accessories: Vec<Accessory<'a>>,
	pub evaluation_type: DataType<'a>,
}

impl<'a> NodeConstruct<'a> for Spanned<Accessor<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.accessor(self)
	}
}

impl<'a> fmt::Display for Accessor<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.expression)?;
		self.accessories.iter().try_for_each(|accessory| write!(f, ".{}", accessory))
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct AccessorTarget<'a>(pub &'a str);

impl<'a> AccessorTarget<'a> {
	pub fn prefixes(&self) -> impl Iterator<Item=AccessorTarget<'a>> {
		let AccessorTarget(accessor) = self;
		std::iter::once(AccessorTarget::default())
			.chain(PrefixSplit::new(accessor, '.')
				.map(|accessor| AccessorTarget(accessor)))
	}

	pub fn split(&self) -> impl DoubleEndedIterator<Item=Identifier<'a>> {
		let AccessorTarget(accessor) = self;
		accessor.split('.').filter(|string| !string.is_empty())
			.map(|identifier| Identifier(identifier))
	}
}

impl<'a> fmt::Display for AccessorTarget<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let AccessorTarget(accessor) = self;
		write!(f, "{}", accessor)
	}
}

#[derive(Debug, Clone)]
pub enum Accessory<'a> {
	FunctionCall(Spanned<FunctionCall<'a>>),
	Field(Spanned<Identifier<'a>>),
}

impl<'a> fmt::Display for Accessory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Accessory::FunctionCall(function_call) => write!(f, "{}", function_call),
			Accessory::Field(field) => write!(f, "{}", field),
		}
	}
}
