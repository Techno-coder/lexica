use crate::basic::{Branch, Compound, Direction, Item, Reversibility, Statement};
use crate::context::Context;
use crate::error::Diagnostic;
use crate::node::Variable;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationFrame};

#[derive(Debug)]
pub struct EvaluationContext<'a> {
	context: &'a Context,
	reversibility: Reversibility,
	frames: Vec<EvaluationFrame>,
	stack: DropStack,
}

// TODO: Add reverse execution
impl<'a> EvaluationContext<'a> {
	pub fn new(context: &'a Context, reversibility: Reversibility, frame: EvaluationFrame) -> Self {
		EvaluationContext {
			context,
			reversibility,
			frames: vec![frame],
			stack: DropStack::default(),
		}
	}

	pub fn advance(&mut self) -> Result<Option<Item>, Diagnostic> {
		let frame = self.frame();
		let node = frame.context.node(&frame.function);
		if frame.context.next_statement == node.statements.len() {
			frame.context.next_statement = 0;
			match self.branch(Direction::Advance)? {
				Some(item) => match self.frames.last_mut() {
					Some(frame) => match &frame.context.statement(&frame.function).node {
						Statement::Binding(variable, Compound::FunctionCall(_, _)) => {
							frame.context.insert(variable.clone(), item);
							frame.context.next_statement += 1;
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
				self.frame().context.next_statement += 1;
			}
			Ok(None)
		}
	}

	/// Executes the current statement. Returns true if a function call was invoked.
	fn execute(&mut self, direction: Direction) -> Result<bool, Diagnostic> {
		let frame = self.frames.last_mut()
			.expect("Evaluation frame stack is empty");
		let context = &mut frame.context;
		let statement = context.statement(&frame.function);
		let span = statement.span;

		match &statement.node {
			Statement::Binding(_, Compound::FunctionCall(function_path, arguments)) => {
				let function = crate::basic::function(&self.context,
					function_path, self.reversibility)?;
				let mut frame = EvaluationFrame::new(function.clone());
				arguments.iter().map(|argument| context.value(argument))
					.enumerate().for_each(|(index, argument)| frame.context
					.insert(Variable::new_temporary(index), argument.clone()));
				self.frames.push(frame);
				return Ok(true);
			}
			Statement::Binding(variable, compound) =>
				super::binding::binding(context, variable, compound),
			Statement::Mutation(mutation, location, value) => super::mutation::mutation(context,
				&mut self.stack, mutation, location, value, direction),
			Statement::ImplicitDrop(location) => Ok(match direction {
				Direction::Advance =>
					self.stack.drop(context.location(location, |_, item| item.clone())),
				Direction::Reverse => {
					let stack_item = self.stack.restore();
					context.location(location, |_, item| *item = stack_item);
				}
			}),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, span))).map(|_| false)
	}

	/// Evaluates the current node branch. Returns an item on a return branch.
	fn branch(&mut self, direction: Direction) -> Result<Option<Item>, Diagnostic> {
		let frame = self.frame();
		let node = frame.context.node(&frame.function);
		let branch = &node[direction];

		match &branch.node {
			Branch::Jump(target) => frame.context.current_node = *target,
			Branch::Divergence(divergence) => {
				let discriminant = frame.context.value(&divergence.discriminant).clone().into();
				let target = divergence.branches.iter().find(|(value, _)| value == &discriminant)
					.map(|(_, target)| *target).unwrap_or(divergence.default);
				frame.context.current_node = target;
			}
			Branch::Return(value) => {
				let item = frame.context.value(value).clone();
				self.frames.pop().unwrap();
				return Ok(Some(item));
			}
			Branch::Unreachable => {
				let error = EvaluationError::UnreachableBranch;
				return Err(Diagnostic::new(Spanned::new(error, branch.span)));
			}
		}
		Ok(None)
	}

	fn frame(&mut self) -> &mut EvaluationFrame {
		self.frames.last_mut().expect("Evaluation frame stack is empty")
	}
}

#[derive(Debug, Default)]
pub struct DropStack {
	stack: Vec<Item>,
}

impl DropStack {
	pub fn drop(&mut self, item: Item) {
		self.stack.push(item);
	}

	pub fn restore(&mut self) -> Item {
		self.stack.pop().expect("Cannot restore from empty drop stack")
	}
}
