use std::fmt;

use crate::interpreter::Primitive;
use crate::source::Spanned;

use super::{BinaryOperation, DataType, ExpressionBlock, FunctionCall, NodeConstruct, NodeVisitor,
	VariableTarget, WhenConditional};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Unit,
	Variable(VariableTarget<'a>),
	Primitive(Primitive),
	BinaryOperation(Spanned<BinaryOperation<'a>>),
	WhenConditional(Spanned<WhenConditional<'a>>),
	ExpressionBlock(Spanned<ExpressionBlock<'a>>),
	FunctionCall(Spanned<FunctionCall<'a>>),
}

#[derive(Debug, Clone)]
pub struct ExpressionNode<'a> {
	pub expression: Box<Expression<'a>>,
	pub evaluation_type: DataType<'a>,
}

impl<'a> NodeConstruct<'a> for Spanned<ExpressionNode<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.expression(self)
	}
}

impl<'a> From<Expression<'a>> for ExpressionNode<'a> {
	fn from(expression: Expression<'a>) -> Self {
		let evaluation_type = match expression {
			Expression::Unit => DataType::UNIT_TYPE,
			_ => DataType::default(),
		};

		let expression = Box::new(expression);
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
		match self.expression.as_ref() {
			Expression::Unit => write!(f, "()"),
			Expression::Variable(target) => write!(f, "{}", target),
			Expression::Primitive(primitive) => write!(f, "{}", primitive),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
			Expression::WhenConditional(when_conditional) => write!(f, "{}", when_conditional),
			Expression::ExpressionBlock(expression_block) => write!(f, "{}", expression_block),
			Expression::FunctionCall(function_call) => write!(f, "{}", function_call),
		}
	}
}
