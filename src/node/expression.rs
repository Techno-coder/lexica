use std::fmt;

use crate::interpreter::Primitive;
use crate::source::Spanned;

use super::{BinaryOperation, FunctionCall, Identifier, NodeConstruct, NodeVisitor};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Variable(Identifier<'a>),
	Primitive(Primitive),
	BinaryOperation(Box<BinaryOperation<'a>>),
	FunctionCall(Box<FunctionCall<'a>>),
}

impl<'a> NodeConstruct<'a> for Spanned<Expression<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression(self)
	}
}

impl<'a> fmt::Display for Expression<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Variable(identifier) => write!(f, "{}", identifier),
			Expression::Primitive(primitive) => write!(f, "{}", primitive),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
			Expression::FunctionCall(function_call) => write!(f, "{}", function_call),
		}
	}
}
