use std::fmt;

use crate::interpreter::Primitive;
use crate::source::Spanned;

use super::{BinaryOperation, DataType, FunctionCall, NodeConstruct, NodeVisitor, VariableTarget};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Variable(VariableTarget<'a>),
	Primitive(Primitive),
	BinaryOperation(Box<BinaryOperation<'a>>),
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

impl<'a> Spanned<ExpressionNode<'a>> {
	pub fn binary_operation(&mut self) -> Spanned<&mut BinaryOperation<'a>> {
		let span = self.span.clone();
		match &mut self.expression {
			Expression::BinaryOperation(operation) => {
				let operation = Box::as_mut(operation);
				Spanned::new(operation, span)
			}
			_ => panic!("Expression is not a binary operation")
		}
	}

	pub fn function_call(&mut self) -> Spanned<&mut FunctionCall<'a>> {
		let span = self.span.clone();
		match &mut self.expression {
			Expression::FunctionCall(function_call) => {
				let function_call = Box::as_mut(function_call);
				Spanned::new(function_call, span)
			}
			_ => panic!("Expression is not a function call")
		}
	}
}

impl<'a> From<Expression<'a>> for ExpressionNode<'a> {
	fn from(expression: Expression<'a>) -> Self {
		let evaluation_type = DataType::default();
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
			Expression::Variable(target) => write!(f, "{}", target),
			Expression::Primitive(primitive) => write!(f, "{}", primitive),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
			Expression::FunctionCall(function_call) => write!(f, "{}", function_call),
		}
	}
}
