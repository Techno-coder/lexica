use std::fmt;

use crate::node::{BinaryOperator, DataType};
use crate::source::{Span, Spanned};

use super::{BinaryOperation, Expression, FunctionCall};

/// Values are expressions that may need to be stored in a temporary
/// before assignment or use in compound expressions.
#[derive(Debug, Clone)]
pub enum Value<'a> {
	Uninitialized(Span),
	Expression(Spanned<Expression<'a>>),
	FunctionCall(Spanned<FunctionCall<'a>>),
	BinaryOperation(Spanned<BinaryOperation<'a>>),
}

impl<'a> Value<'a> {
	pub fn data_type(&self) -> DataType<'a> {
		match self {
			Value::Uninitialized(_) => DataType::EMPTY,
			Value::Expression(expression) => expression.data_type(),
			Value::FunctionCall(function_call) => function_call.evaluation_type.clone(),
			Value::BinaryOperation(operation) => match operation.operator.node {
				BinaryOperator::Equal => DataType::BOOLEAN,
				_ => {
					let left = operation.left.node.data_type();
					let right = operation.left.node.data_type();
					assert_eq!(left, right);
					left
				}
			}
		}
	}

	pub fn span(&self) -> Span {
		match self {
			Value::Uninitialized(span) => span.clone(),
			Value::Expression(expression) => expression.span,
			Value::FunctionCall(function_call) => function_call.span,
			Value::BinaryOperation(operation) => operation.span,
		}
	}
}

impl<'a> fmt::Display for Value<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Uninitialized(_) => write!(f, "<!>"),
			Value::Expression(expression) => write!(f, "{}", expression),
			Value::FunctionCall(function_call) => write!(f, "{}", function_call),
			Value::BinaryOperation(operation) => write!(f, "{}", operation),
		}
	}
}
