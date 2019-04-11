use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Expression, Variable};

#[derive(Debug)]
pub struct Binding<'a> {
	pub variable: Variable<'a>,
	pub expression: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for Binding<'a> {
	fn dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		vec![
			Dependency::advance(&self.expression)
		]
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		// TODO: Add to drop stack, check data types
		let value = context.evaluation(&self.expression);
		context.register_binding(self.variable.identifier.clone(), *value);
		Ok(ExecutionStep::Void)
	}

	fn reverse_dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency> {
		self.dependencies(_context)
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		context.invalidate_binding(&self.variable.identifier);
		Ok(ExecutionStep::Void)
	}
}

impl<'a> fmt::Display for Binding<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "let {} = {}", self.variable, self.expression)
	}
}
