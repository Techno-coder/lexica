use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Expression, Statement, Variable};

#[derive(Debug)]
pub struct Function<'a> {
	pub parameters: Vec<Variable<'a>>,
	pub statements: Vec<Statement<'a>>,
	pub return_value: Expression<'a>,
}

// TODO: Replace with FunctionCall as this is a definition
impl<'a> NodeConstruct<'a> for Function<'a> {
	fn dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		self.statements.iter().map(|node| Dependency::advance(node))
		    .chain(::std::iter::once(Dependency::advance(&self.return_value))).collect()
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		let value = context.evaluation(&self.return_value);
		Ok(ExecutionStep::Value(*value))
	}

	fn reverse_dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency> {
		self.dependencies(context)
	}

	fn reverse(&self, _context: &mut Context) -> Result<ExecutionStep, ()> {
		Ok(ExecutionStep::Void)
	}
}

impl<'a> fmt::Display for Function<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use std::fmt::Write;
		use crate::display::IndentWriter;

		write!(f, "(")?;
		let split = self.parameters.split_last();
		if let Some((last, rest)) = split {
			for parameter in rest {
				write!(f, "{}, ", parameter)?;
			}
			write!(f, "{}", last)?;
		}
		writeln!(f, ") {{")?;

		let mut indent = IndentWriter::wrap(f);
		for statement in &self.statements {
			writeln!(indent, "{}", statement)?;
		}
		writeln!(indent, "{}", self.return_value)?;
		write!(f, "}}")
	}
}
