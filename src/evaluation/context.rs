use crate::basic::{Branch, Statement};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationFrame};

#[derive(Debug)]
pub struct EvaluationContext {
	frames: Vec<EvaluationFrame>,
}

// TODO: Add reverse execution
impl EvaluationContext {
	pub fn new(frame: EvaluationFrame) -> Self {
		EvaluationContext { frames: vec![frame] }
	}

	pub fn advance(&mut self) -> Result<(), Diagnostic> {
		if !self.advance_branch()? {
			self.advance_execute()?;
			self.frame().context.next_statement += 1;
		}
		Ok(())
	}

	fn advance_execute(&mut self) -> Result<(), Diagnostic> {
		let frame = self.frame();
		let statement = frame.context.statement(&frame.function);
		match &statement.node {
			Statement::Binding(variable, compound) =>
				super::binding::binding(&mut frame.context, variable, compound),
			Statement::Mutation(mutation, location, value) =>
				super::mutation::mutation(&mut frame.context, mutation, location, value),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, statement.span)))
	}

	/// Evaluates the node branch if the advance endpoint is reached.
	/// Returns true if a node change was performed.
	fn advance_branch(&mut self) -> Result<bool, Diagnostic> {
		let frame = self.frame();
		let node = frame.context.node(&frame.function);
		if frame.context.next_statement == node.statements.len() {
			frame.context.next_statement = 0;
			match &node.advance.node {
				Branch::Jump(target) => frame.context.current_node = *target,
				Branch::Divergence(divergence) => {
					let discriminant = frame.context.value(&divergence.discriminant).clone().into();
					let target = divergence.branches.iter().find(|(value, _)| value == &discriminant)
						.map(|(_, target)| *target).unwrap_or(divergence.default);
					frame.context.current_node = target;
				}
				Branch::Return(value) => {
					// TODO: Apply return value to parent function
					println!("Function value: {:#?}", frame.context.value(value));
					unimplemented!()
				}
				Branch::Unreachable => {
					let error = EvaluationError::UnreachableBranch;
					return Err(Diagnostic::new(Spanned::new(error, node.advance.span)));
				}
			}
			return Ok(true);
		}
		Ok(false)
	}

	fn frame(&mut self) -> &mut EvaluationFrame {
		self.frames.last_mut().expect("Evaluation frame stack is empty")
	}
}
