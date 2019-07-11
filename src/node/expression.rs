use std::fmt;

use crate::interpreter::Primitive;
use crate::source::Spanned;

use super::{BinaryOperation, DataType, FunctionCall, NodeConstruct, NodeVisitor, VariableTarget,
            WhenConditional};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Unit,
	Variable(VariableTarget<'a>),
	Primitive(Primitive),
	BinaryOperation(Box<BinaryOperation<'a>>),
	WhenConditional(Box<WhenConditional<'a>>),
	FunctionCall(Box<FunctionCall<'a>>),
}

#[derive(Debug, Clone)]
pub struct ExpressionNode<'a> {
	pub expression: Expression<'a>,
	pub evaluation_type: DataType<'a>,
}

impl<'a> NodeConstruct<'a> for Spanned<ExpressionNode<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression(self)
	}
}

macro_rules! forward {
    ($identifier: ident, $type: ident) => {
		pub fn $identifier(&mut self) -> Spanned<&mut $type<'a>> {
			let span = self.span.clone();
			match &mut self.expression {
				Expression::$type($identifier) => {
					let $identifier = Box::as_mut($identifier);
					Spanned::new($identifier, span)
				}
				_ => panic!("Expression is not a {}", stringify!($identifier))
			}
		}
    };
}

impl<'a> Spanned<ExpressionNode<'a>> {
	forward!(binary_operation, BinaryOperation);
	forward!(when_conditional, WhenConditional);
	forward!(function_call, FunctionCall);
}

impl<'a> From<Expression<'a>> for ExpressionNode<'a> {
	fn from(expression: Expression<'a>) -> Self {
		let evaluation_type = match expression {
			Expression::Unit => DataType::UNIT_TYPE,
			_ => DataType::default(),
		};
		ExpressionNode { expression, evaluation_type }
	}
}

impl<'a> AsRef<Expression<'a>> for ExpressionNode<'a> {
	fn as_ref(&self) -> &Expression<'a> {
		&self.expression
	}
}

impl<'a> AsMut<Expression<'a>> for ExpressionNode<'a> {
	fn as_mut(&mut self) -> &mut Expression<'a> {
		&mut self.expression
	}
}

impl<'a> fmt::Display for ExpressionNode<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.expression {
			Expression::Unit => write!(f, "()"),
			Expression::Variable(target) => write!(f, "{}", target),
			Expression::Primitive(primitive) => write!(f, "{}", primitive),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
			Expression::WhenConditional(when_conditional) => write!(f, "{}", when_conditional),
			Expression::FunctionCall(function_call) => write!(f, "{}", function_call),
		}
	}
}
