use std::fmt;

use super::{Context, ExecutionStep, Identifier, NodeConstruct};

#[derive(Debug)]
pub struct Swap<'a> {
	pub left: Identifier<'a>,
	pub right: Identifier<'a>,
}

impl<'a> NodeConstruct<'a> for Swap<'a> {
	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		let left = *context.binding_value(&self.left);
		let right = *context.binding_value(&self.right);
		context.register_binding(self.left.clone(), right);
		context.register_binding(self.right.clone(), left);
		Ok(ExecutionStep::Void)
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		self.execute(context)
	}
}

impl<'a> fmt::Display for Swap<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} <=> {}", self.left, self.right)
	}
}
