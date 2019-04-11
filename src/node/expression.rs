use std::fmt;

use super::NodeConstruct;
use super::{BinaryOperation, Identifier};

#[derive(Debug)]
pub enum Expression<'a> {
	Variable(Identifier<'a>),
	LiteralInteger(i64),
	BinaryOperation(Box<BinaryOperation<'a>>),
}

impl<'a> NodeConstruct<'a> for Expression<'a> {
}

impl<'a> fmt::Display for Expression<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Expression::Variable(identifier) => write!(f, "{}", identifier),
			Expression::LiteralInteger(integer) => write!(f, "{}", integer),
			Expression::BinaryOperation(operation) => write!(f, "{}", operation),
		}
	}
}
