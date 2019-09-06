use std::fmt;

use crate::source::Spanned;
use crate::utility::PrefixSplit;

use super::{DataType, ExpressionNode, FunctionCall, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub struct Accessor<'a> {
	pub expression: Spanned<ExpressionNode<'a>>,
	pub target: Spanned<AccessorTarget<'a>>,
	pub function_call: Option<Spanned<FunctionCall<'a>>>,
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
		if !self.target.is_empty() {
			write!(f, ".{}", self.target)?;
		}

		if let Some(function_call) = &self.function_call {
			write!(f, ".{}", function_call)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct AccessorTarget<'a>(pub &'a str);

impl<'a> AccessorTarget<'a> {
	pub fn is_empty(&self) -> bool {
		let AccessorTarget(accessor) = self;
		accessor.is_empty()
	}

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
