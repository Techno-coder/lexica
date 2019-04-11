use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::Expression;

#[derive(Debug)]
pub enum BinaryOperator {
	Equal,
	Plus,
	Minus,
}

impl fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let operator = match self {
			BinaryOperator::Equal => "==",
			BinaryOperator::Plus => "+",
			BinaryOperator::Minus => "-",
		};
		write!(f, "{}", operator)
	}
}

#[derive(Debug)]
pub struct BinaryOperation<'a> {
	pub left: Expression<'a>,
	pub right: Expression<'a>,
	pub operator: BinaryOperator,
}

impl<'a> NodeConstruct<'a> for BinaryOperation<'a> {
	fn dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency> {
		vec![
			Dependency::advance(&self.left),
			Dependency::advance(&self.right)
		]
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		let left = context.evaluation(&self.left);
		let right = context.evaluation(&self.right);
		let value = match self.operator {
			BinaryOperator::Equal => if left == right { 1 } else { 0 },
			BinaryOperator::Plus => left + right,
			BinaryOperator::Minus => left - right,
		};
		Ok(ExecutionStep::Value(value))
	}

	fn reverse_dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency> {
		self.dependencies(_context)
	}

	fn reverse(&'a self, _context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		Ok(ExecutionStep::Void)
	}
}

impl<'a> fmt::Display for BinaryOperation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} {} {}", self.left, self.operator, self.right)
	}
}
