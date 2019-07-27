use hashbrown::{HashMap, HashSet};

use crate::node::*;
use crate::source::{ErrorCollate, Span, Spanned};

use super::ExpositionError;

type DropFrame<'a> = HashSet<VariableTarget<'a>>;
type GenerationFrame<'a> = HashMap<Identifier<'a>, usize>;
type Result<'a> = std::result::Result<(), ErrorCollate<Spanned<ExpositionError<'a>>>>;

/// Verifies that all variable identifiers and targets are valid.
/// Additionally shadows and distinguishes variable targets.
/// Checks that explicitly dropped variables are no longer used.
#[derive(Debug)]
pub struct VariableExposition<'a> {
	generation_frames: Vec<GenerationFrame<'a>>,
	drop_frames: Vec<DropFrame<'a>>,
}

impl<'a> VariableExposition<'a> {
	pub fn generation_frame(&mut self) -> &mut GenerationFrame<'a> {
		self.generation_frames.last_mut().expect("Generation frame does not exist")
	}

	pub fn drop_frame(&mut self) -> &mut DropFrame<'a> {
		self.drop_frames.last_mut().expect("Drop frame does not exist")
	}

	pub fn register_target(&mut self, target: &mut VariableTarget<'a>) {
		let VariableTarget(identifier, generation) = target;
		match self.generation_frame().get_mut(identifier) {
			Some(current_generation) => {
				*current_generation += 1;
				return *generation = *current_generation;
			}
			None => self.generation_frame().insert(identifier.clone(), 0),
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
				return self.check_alive(target, span);
			}
		}

		let undefined_error = ExpositionError::UndefinedVariable(identifier.clone());
		Err(Spanned::new(undefined_error, span).into())
	}

	pub fn check_alive(&self, target: &VariableTarget<'a>, span: Span) -> Result<'a> {
		for frame in self.drop_frames.iter().rev() {
			if frame.contains(target) {
				let error = ExpositionError::DroppedVariable(target.clone());
				return Err(Spanned::new(error, span).into());
			}
		}
		Ok(())
	}
}

impl<'a> NodeVisitor<'a> for VariableExposition<'a> {
	type Result = Result<'a>;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		let mut error_collate = ErrorCollate::new();
		for function in &mut syntax_unit.functions.values_mut() {
			if let Err(errors) = function.accept(self) {
				error_collate.combine(errors);
			}

			*self = Self::default();
		}
		error_collate.collapse(())
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.parameters.iter_mut()
			.for_each(|parameter| self.register_target(&mut parameter.target));
		function.expression_block.accept(self)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let expression_span = expression.span.clone();
		match expression.node.as_mut() {
			Expression::Unit | Expression::Primitive(_) => Ok(()),
			Expression::Variable(target) => self.resolve_target_span(target, expression_span),
			Expression::BinaryOperation(binary_operation) => binary_operation.accept(self),
			Expression::WhenConditional(when_conditional) => when_conditional.accept(self),
			Expression::ExpressionBlock(expression_block) => expression_block.accept(self),
			Expression::FunctionCall(function_call) => function_call.accept(self),
		}
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self)?;
		expression_block.expression.accept(self)
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		block.statements.iter_mut().try_for_each(|statement| statement.accept(self))
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
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
		conditional_loop.block.accept(self)?;
		self.generation_frames.pop();
		Ok(())
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		self.resolve_target(&mut explicit_drop.target)?;
		self.drop_frame().insert(explicit_drop.target.node.clone());
		explicit_drop.expression.accept(self)
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|argument| argument.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(left, right) => {
				self.resolve_target(left)?;
				self.resolve_target(right)
			}
			Mutation::AddAssign(target, expression) |
			Mutation::MinusAssign(target, expression) |
			Mutation::MultiplyAssign(target, expression) => {
				self.resolve_target(target)?;
				expression.accept(self)
			}
		}
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		let mut drop_union = HashSet::new();
		for branch in &mut when_conditional.branches {
			self.drop_frames.push(DropFrame::new());
			branch.condition.accept(self)?;
			branch.end_condition.as_mut().unwrap().accept(self)?;

			self.generation_frames.push(GenerationFrame::new());
			branch.expression_block.accept(self)?;
			self.generation_frames.pop();
			drop_union.extend(self.drop_frames.pop().unwrap());
		}

		self.drop_frame().extend(drop_union);
		Ok(())
	}
}

impl<'a> Default for VariableExposition<'a> {
	fn default() -> Self {
		Self {
			generation_frames: vec![GenerationFrame::new()],
			drop_frames: vec![DropFrame::new()],
		}
	}
}
