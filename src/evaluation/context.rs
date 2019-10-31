use crate::basic::{Branch, Statement};
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{EvaluationError, EvaluationFrame};

#[derive(Debug)]
pub struct Context {
	frames: Vec<EvaluationFrame>,
}

impl Context {
	pub fn new(frame: EvaluationFrame) -> Self {
		Context { frames: vec![frame] }
	}

	pub fn advance(&mut self) -> Result<(), Diagnostic> {
		self.advance_execute()?;
		let frame = self.frame();
		frame.context.next_statement += 1;
		self.advance_branch()
	}

	fn advance_execute(&mut self) -> Result<(), Diagnostic> {
		let frame = self.frame();
		let statement = frame.context.statement(&frame.function);
		match &statement.node {
			Statement::Binding(variable, compound) =>
				super::binding::binding(&mut frame.context, variable, compound),
			Statement::Mutation(mutation, variable, value) =>
				super::mutation::mutation(&mut frame.context, mutation, variable, value),
		}.map_err(|error| Diagnostic::new(Spanned::new(error, statement.span)))
	}

	fn advance_branch(&mut self) -> Result<(), Diagnostic> {
		let frame = self.frame();
		let node = frame.context.node(&frame.function);
		if frame.context.next_statement == node.statements.len() {
			frame.context.next_statement = 0;
			match &node.advance.node {
				Branch::Jump(target) => frame.context.current_node = *target,
				Branch::Divergence(divergence) => {
					let discriminant = frame.context.value(&divergence.discriminant).into();
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
		}
		Ok(())
	}

	fn frame(&mut self) -> &mut EvaluationFrame {
		self.frames.last_mut().expect("Evaluation frame stack is empty")
	}
}
