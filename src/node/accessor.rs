use std::fmt;

use crate::source::{Spanned, Span};

use super::{ExpressionNode, FunctionCall, Identifier, NodeConstruct, NodeVisitor, DataType};

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

#[derive(Debug, Clone)]
pub enum Accessory<'a> {
	FunctionCall(Spanned<FunctionCall<'a>>),
	Field(Spanned<Identifier<'a>>),
}

impl<'a> Accessory<'a> {
	pub fn span(&self) -> Span {
		match self {
			Accessory::FunctionCall(function_call) => function_call.span,
			Accessory::Field(field) => field.span,
		}
	}
}

impl<'a> fmt::Display for Accessory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Accessory::FunctionCall(function_call) => write!(f, "{}", function_call),
			Accessory::Field(field) => write!(f, "{}", field),
		}
	}
}
