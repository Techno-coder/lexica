use crate::basic::{Branch, Direction, Item, Statement};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationFrame};

#[derive(Debug)]
pub struct EvaluationContext {
	frames: Vec<EvaluationFrame>,
	stack: DropStack,
}

// TODO: Add reverse execution
impl EvaluationContext {
	pub fn new(frame: EvaluationFrame) -> Self {
		EvaluationContext {
			frames: vec![frame],
			stack: DropStack::default(),
		}
	}

	pub fn advance(&mut self) -> Result<Option<Item>, Diagnostic> {
		let frame = self.frame();
		let node = frame.context.node(&frame.function);
		if frame.context.next_statement == node.statements.len() {
			frame.context.next_statement = 0;
			self.branch(Direction::Advance)
		} else {
			self.execute(Direction::Advance)?;
			self.frame().context.next_statement += 1;
			Ok(None)
		}
	}

	fn execute(&mut self, direction: Direction) -> Result<(), Diagnostic> {
		let frame = self.frames.last_mut()
			.expect("Evaluation frame stack is empty");
		let context = &mut frame.context;
		let statement = context.statement(&frame.function);

		match &statement.node {
			Statement::Binding(variable, compound) =>
				super::binding::binding(context, variable, compound),
			Statement::Mutation(mutation, location, value) => super::mutation::mutation(context,
				&mut self.stack, mutation, location, value, direction),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, statement.span)))
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
