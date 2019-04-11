use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{BinaryOperation, Identifier};

#[derive(Debug)]
pub enum Expression<'a> {
	Variable(Identifier<'a>),
	LiteralInteger(i64),
	BinaryOperation(Box<BinaryOperation<'a>>),
}

impl<'a> NodeConstruct<'a> for Expression<'a> {
	fn dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		match self {
			Expression::BinaryOperation(operation) => operation.dependencies(context),
			_ => Vec::new()
		}
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		Ok(match self {
			Expression::Variable(variable) => ExecutionStep::Value(*context.binding_value(variable)),
			Expression::LiteralInteger(integer) => ExecutionStep::Value(*integer),
			Expression::BinaryOperation(operation) => return operation.execute(context),
		})
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match self {
			Expression::BinaryOperation(operation) => operation.reverse(context),
			_ => Ok(ExecutionStep::Void),
		}
	}
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
