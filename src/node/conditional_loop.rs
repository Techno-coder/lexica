use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Expression, Statement};

#[derive(Debug)]
pub struct ConditionalLoop<'a> {
	pub start_condition: Option<Expression<'a>>,
	pub end_condition: Expression<'a>,
	pub statements: Vec<Statement<'a>>,
}

impl<'a> NodeConstruct<'a> for ConditionalLoop<'a> {
	fn dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		if context.has_evaluated(&self.end_condition) {
			context.invalidate_evaluation(&self.end_condition);
			self.statements.iter().map(|node| Dependency::advance(node)).collect()
		} else {
			vec![Dependency::advance(&self.end_condition)]
		}
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		if context.has_evaluated(&self.end_condition) {
			let value = context.evaluation(&self.end_condition);
			if *value == 1 {
				return Ok(ExecutionStep::Void);
			}
		}
		Ok(ExecutionStep::Repeat)
	}

	fn reverse_dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		match &self.start_condition {
			Some(condition) => {
				if context.has_evaluated(condition) {
					context.invalidate_evaluation(condition);
					self.statements.iter().rev().map(|node| Dependency::advance(node)).collect()
				} else {
					vec![Dependency::advance(condition)]
				}
			}
			None => {
				// TODO: Proper reversibility clause
				panic!("Cannot reverse")
			}
		}
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match &self.start_condition {
			Some(condition) => {
				if context.has_evaluated(condition) {
					let value = context.evaluation(condition);
					if *value == 1 {
						return Ok(ExecutionStep::Void);
					}
				}
				Ok(ExecutionStep::Repeat)
			}
			None => {
				// TODO: Proper reversibility clause
				panic!("Cannot reverse")
			}
		}
	}
}

impl<'a> fmt::Display for ConditionalLoop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use std::fmt::Write;
		use crate::display::IndentWriter;

		if let Some(start_condition) = &self.start_condition {
			writeln!(f, "loop {} => {} {{", start_condition, self.end_condition)?;
		} else {
			writeln!(f, "loop {} {{", self.end_condition)?;
		}

		let mut indent = IndentWriter::wrap(f);
		for statement in &self.statements {
			writeln!(indent, "{}", statement)?;
		}
		write!(f, "}}")
	}
}
