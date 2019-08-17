use std::fmt;

use crate::interpreter::Primitive;
use crate::node::{BinaryOperator, DataType, Identifier, Variable};
use crate::source::Spanned;

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Unit,
	Variable(Variable<'a>),
	Primitive(Primitive),
}

impl<'a> Expression<'a> {
	pub fn data_type(&self) -> DataType<'a> {
		match self {
			Expression::Unit => DataType::UNIT,
			Expression::Variable(variable) => variable.data_type.clone(),
			Expression::Primitive(primitive) => {
				let identifier = Identifier(primitive.size().to_string());
				DataType::new(identifier)
			}
		}
	}
}

impl<'a> fmt::Display for Expression<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Unit => write!(f, "()"),
			Expression::Variable(variable) => write!(f, "{}", variable.target),
			Expression::Primitive(primitive) => write!(f, "{}", primitive),
		}
	}
}

#[derive(Debug, Clone)]
pub struct BinaryOperation<'a> {
	pub left: Spanned<Expression<'a>>,
	pub right: Spanned<Expression<'a>>,
	pub operator: Spanned<BinaryOperator>,
}

impl<'a> fmt::Display for BinaryOperation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {} {}", self.left, self.operator, self.right)
	}
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
	pub function: Spanned<Identifier<'a>>,
	pub arguments: Vec<Spanned<Expression<'a>>>,
	pub evaluation_type: DataType<'a>,
}

impl<'a> fmt::Display for FunctionCall<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}(", self.function)?;

		let split = self.arguments.split_last();
		if let Some((last, rest)) = split {
			rest.iter().try_for_each(|argument| write!(f, "{}, ", argument))?;
			write!(f, "{}", last)?;
		}

		write!(f, ")")
	}
}
