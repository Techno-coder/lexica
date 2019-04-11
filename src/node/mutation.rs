use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Expression, Identifier};

#[derive(Debug)]
pub enum Mutation<'a> {
	AddAssign(Identifier<'a>, Expression<'a>),
}

impl<'a> NodeConstruct<'a> for Mutation<'a> {
	fn dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		match self {
			Mutation::AddAssign(_, expression) => vec![Dependency::advance(expression)],
		}
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match self {
			Mutation::AddAssign(identifier, expression) => {
				let current_value = context.binding_value(identifier);
				let evaluation = context.evaluation(expression);
				context.register_binding(identifier.clone(), current_value + evaluation);
				Ok(ExecutionStep::Void)
			}
		}
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match self {
			Mutation::AddAssign(identifier, expression) => {
				let current_value = context.binding_value(identifier);
				let evaluation = context.evaluation(expression);
				context.register_binding(identifier.clone(), current_value - evaluation);
				Ok(ExecutionStep::Void)
			}
		}
	}
}

impl<'a> fmt::Display for Mutation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Mutation::AddAssign(identifier, expression) => write!(f, "{} += {}", identifier, expression),
		}
	}
}
