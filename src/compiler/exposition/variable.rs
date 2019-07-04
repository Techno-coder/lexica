use std::collections::HashMap;

use crate::node::*;
use crate::source::{ErrorCollate, Span, Spanned};

use super::ExpositionError;

type GenerationFrame<'a> = HashMap<Identifier<'a>, usize>;
type Result<'a> = std::result::Result<(), ErrorCollate<Spanned<ExpositionError<'a>>>>;

/// Verifies that all variable identifiers and targets are valid.
/// Additionally shadows and distinguishes variable targets.
#[derive(Debug)]
pub struct VariableExposition<'a> {
	generation_frames: Vec<GenerationFrame<'a>>,
}

impl<'a> VariableExposition<'a> {
	pub fn frame(&mut self) -> &mut GenerationFrame<'a> {
		self.generation_frames.last_mut().expect("Generation frame does not exist")
	}

	pub fn register_target(&mut self, target: &mut VariableTarget<'a>) {
		let VariableTarget(identifier, generation) = target;
		match self.frame().get_mut(identifier) {
			Some(current_generation) => {
				*current_generation += 1;
				return *generation = *current_generation;
			}
			None => self.frame().insert(identifier.clone(), 0),
		};
	}

	pub fn resolve_target(&self, target: &mut Spanned<VariableTarget<'a>>) -> Result<'a> {
		let target_span = target.span.clone();
		self.resolve_target_span(target, target_span)
	}

	pub fn resolve_target_span(&self, target: &mut VariableTarget<'a>, span: Span) -> Result<'a> {
		let VariableTarget(identifier, generation) = target;
		for frame in self.generation_frames.iter().rev() {
			if let Some(target_generation) = frame.get(identifier) {
				*generation = *target_generation;
				return Ok(());
			}
		}

		let undefined_error = ExpositionError::UndefinedVariable(identifier.clone());
		Err(Spanned::new(undefined_error, span).into())
	}
}

impl<'a> NodeVisitor<'a> for VariableExposition<'a> {
	type Result = Result<'a>;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self)?;
		operation.right.accept(self)?;
		Ok(())
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self)?;
		Ok(self.register_target(&mut binding.variable.node.target))
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.start_condition.as_mut().unwrap().accept(self)?;
		conditional_loop.end_condition.accept(self)?;

		self.generation_frames.push(GenerationFrame::new());
		conditional_loop.statements.iter_mut().try_for_each(|statement| statement.accept(self))?;
		self.generation_frames.pop();
		Ok(())
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		self.resolve_target(&mut explicit_drop.target)?;
		explicit_drop.expression.accept(self)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let expression_span = expression.span.clone();
		match &mut expression.expression {
			Expression::Primitive(_) => Ok(()),
			Expression::Variable(target) => self.resolve_target_span(target, expression_span),
			Expression::BinaryOperation(_) => expression.binary_operation().accept(self),
			Expression::FunctionCall(_) => expression.function_call().accept(self),
		}
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.parameters.iter_mut()
			.for_each(|parameter| self.register_target(&mut parameter.target));
		function.statements.iter_mut()
			.try_for_each(|statement| statement.accept(self))?;
		function.return_value.accept(self)
	}

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|argument| argument.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(left, right) => {
				self.resolve_target(left)?;
				self.resolve_target(right)
			}
			Mutation::AddAssign(target, expression) |
			Mutation::MultiplyAssign(target, expression) => {
				self.resolve_target(target)?;
				expression.accept(self)
			}
		}
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
		}
	}

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		let mut error_collate = ErrorCollate::new();
		for (_, function) in &mut syntax_unit.functions {
			if let Err(errors) = function.accept(self) {
				error_collate.combine(errors);
			}

			*self = Self::default();
		}
		error_collate.collapse(())
	}
}

impl<'a> Default for VariableExposition<'a> {
	fn default() -> Self {
		let generation_frames = vec![GenerationFrame::new()];
		Self { generation_frames }
	}
}
