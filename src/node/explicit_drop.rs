use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Expression, Identifier};

#[derive(Debug)]
pub struct ExplicitDrop<'a> {
	pub identifier: Identifier<'a>,
	pub expression: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for ExplicitDrop<'a> {
	fn execute(&self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		context.invalidate_binding(&self.identifier);
		Ok(ExecutionStep::Void)
	}

	fn reverse_dependencies(&'a self, _context: &mut Context) -> Vec<Dependency<'a>> {
		vec![
			Dependency::reverse(&self.expression)
		]
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		let value = context.evaluation(&self.expression);
		context.register_binding(self.identifier.clone(), *value);
		Ok(ExecutionStep::Void)
	}
}

impl<'a> fmt::Display for ExplicitDrop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "drop {} = {}", self.identifier, self.expression)
	}
}
