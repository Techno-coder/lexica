use std::sync::Arc;

use crate::basic::{BasicFunction, Branch, Compound, Direction, Discriminant,
	NodeTarget, Reversibility, Statement};
use crate::context::Context;
use crate::error::Diagnostic;
use crate::node::Variable;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationItem, ValueContext, ValueFrame};

#[derive(Debug)]
pub struct EvaluationContext<'a> {
	values: ValueContext,
	functions: Vec<FunctionFrame>,
	reversibility: Reversibility,
	context: &'a Context,
}

impl<'a> EvaluationContext<'a> {
	pub fn new(context: &'a Context, reversibility: Reversibility,
	           function: Arc<BasicFunction>, value: ValueFrame) -> Result<Self, Diagnostic> {
		let (values, functions) = (ValueContext::new(value), vec![FunctionFrame::new(function)]);
		Ok(EvaluationContext { values, functions, reversibility, context })
	}

	pub fn advance(&mut self) -> Result<Option<EvaluationItem>, Diagnostic> {
		let frame = self.frame();
		let node = &frame.function[&frame.node];
		if frame.statement == node.statements.len() {
			frame.statement = 0;
			match self.branch(Direction::Advance)? {
				Some(item) => match self.functions.last_mut() {
					Some(frame) => match &frame.statement().node {
						Statement::Binding(variable, Compound::FunctionCall(_, _)) => {
							self.values.frame().items.insert(variable.clone(), item);
							frame.statement += 1;
							Ok(None)
						}
						_ => panic!("Cannot return into statement that is not function call"),
					},
					None => Ok(Some(item))
				}
				None => Ok(None),
			}
		} else {
			if !self.execute(Direction::Advance)? {
				self.frame().statement += 1;
			}
			Ok(None)
		}
	}

	/// Executes the current statement. Returns true if a function call was invoked.
	fn execute(&mut self, direction: Direction) -> Result<bool, Diagnostic> {
		let frame = self.functions.last_mut().expect("Evaluation function stack is empty");
		let statement = frame.statement();
		let values = &mut self.values;
		match &statement.node {
			Statement::Binding(variable, compound) => match compound {
				Compound::FunctionCall(path, arguments) => {
					let mut frame = ValueFrame::default();
					arguments.iter().map(|argument| values.value(argument)).enumerate()
						.for_each(|(index, argument)| frame.items.insert(Variable::new_temporary(index),
							argument).unwrap_none());
					values.frames.push(frame);

					let function = crate::basic::function(&self.context, path, self.reversibility)?;
					self.functions.push(FunctionFrame::new(function));
					return Ok(true);
				}
				_ => super::binding::binding(&mut self.values, variable, compound),
			}
			Statement::Mutation(mutation, location, value) =>
				super::mutation::mutation(&mut self.values, &self.reversibility,
					direction, mutation, location, value),
			Statement::ImplicitDrop(location) => Ok(match direction {
				Direction::Reverse => *self.values.location(location) = self.values.stack.restore(),
				Direction::Advance => {
					let item = self.values.location(location).clone();
					self.values.stack.drop(item);
				}
			}),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, statement.span))).map(|_| false)
	}

	/// Evaluates the current node branch. Returns an item on a return branch.
	fn branch(&mut self, direction: Direction) -> Result<Option<EvaluationItem>, Diagnostic> {
		let frame = self.functions.last_mut().expect("Evaluation function stack is empty");
		let node = &frame.function[&frame.node];
		let branch = &node[direction];
		match &branch.node {
			Branch::Jump(target) => frame.node = *target,
			Branch::Divergence(divergence) => {
				let item = self.values.value(&divergence.discriminant);
				let discriminant = Discriminant::item(item.collapse().unwrap());
				frame.node = divergence.branches.iter().find(|(value, _)| value == &discriminant)
					.map(|(_, target)| *target).unwrap_or(divergence.default);
			}
			Branch::Return(value) => {
				let item = self.values.value(value);
				self.values.frames.pop().unwrap();
				self.functions.pop().unwrap();
				return Ok(Some(item));
			}
			Branch::Unreachable => {
				let error = EvaluationError::UnreachableBranch;
				return Err(Diagnostic::new(Spanned::new(error, branch.span)));
			}
		}
		Ok(None)
	}

	fn frame(&mut self) -> &mut FunctionFrame {
		self.functions.last_mut().expect("Evaluation function stack is empty")
	}
}

#[derive(Debug)]
struct FunctionFrame {
	node: NodeTarget,
	statement: usize,
	function: Arc<BasicFunction>,
}

impl FunctionFrame {
	fn new(function: Arc<BasicFunction>) -> Self {
		let node = function.component.entry;
		FunctionFrame { node, statement: 0, function }
	}

	fn statement(&self) -> &Spanned<Statement> {
		let node = &self.function[&self.node];
		node.statements.get(self.statement).unwrap_or_else(||
			panic!("Statement index: {}, is invalid in node: {}, of length: {}",
				self.statement, self.node, node.statements.len()))
	}
}
