use std::collections::HashMap;

use polytype::{Context, Type, UnificationError};

use crate::node::*;
use crate::source::{ErrorCollate, Spanned};

use super::TypeResult;

/// Applies type inference and analysis.
#[derive(Debug, Default)]
pub struct InferenceEngine<'a> {
	environment: HashMap<VariableTarget<'a>, Type<Identifier<'a>>>,
	context: Context<Identifier<'a>>,
}

impl<'a> InferenceEngine<'a> {
	pub fn context(self) -> Context<Identifier<'a>> {
		self.context
	}

	// TODO: Use disjoint set structure to improve application
	// TODO: Use mutable reference to modify syntax tree
	pub fn unify(&mut self, mut left: Type<Identifier<'a>>, mut right: Type<Identifier<'a>>)
	             -> Result<(), UnificationError<Identifier<'a>>> {
		super::application::apply(&self.context, &mut left);
		super::application::apply(&self.context, &mut right);
		self.context.unify(&left, &right)
	}
}

impl<'a> NodeVisitor<'a> for InferenceEngine<'a> {
	type Result = TypeResult<'a, ()>;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self)?;
		operation.right.accept(self)?;

		let left = operation.left.evaluation_type.as_ref();
		let right = operation.right.evaluation_type.as_ref();
		Ok(self.unify(left.clone(), right.clone())
			.map_err(|error| Spanned::new(error.into(), operation.span))?)
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self)?;
		if binding.variable.data_type.resolved().is_none() {
			binding.variable.data_type = DataType(self.context.new_variable());
		}

		let binding_type = binding.variable.data_type.as_ref();
		self.unify(binding_type.clone(), binding.expression.evaluation_type.as_ref().clone())
			.map_err(|error| Spanned::new(error.into(), binding.span))?;
		self.environment.insert(binding.variable.target.clone(), binding_type.clone());
		Ok(())
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.end_condition.accept(self)?;
		let start_condition = conditional_loop.start_condition.as_mut().unwrap();
		start_condition.accept(self)?;

		const BOOLEAN_TYPE: Type<Identifier<'static>> = super::application::BOOLEAN_TYPE;
		self.unify(start_condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
			.map_err(|error| Spanned::new(error.into(), start_condition.span))?;
		self.unify(conditional_loop.end_condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
			.map_err(|error| Spanned::new(error.into(), conditional_loop.end_condition.span))?;

		conditional_loop.statements.iter_mut().try_for_each(|statement| statement.accept(self))?;
		Ok(())
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		let identifier_type = self.environment[&explicit_drop.target].clone();
		let expression_type = explicit_drop.expression.evaluation_type.as_ref();
		Ok(self.unify(identifier_type, expression_type.clone())
			.map_err(|error| Spanned::new(error.into(), explicit_drop.span))?)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let evaluation_type = match &mut expression.expression {
			Expression::Variable(target) => DataType(self.environment[target].clone()),
			Expression::Primitive(_) => DataType(self.context.new_variable()),
			Expression::BinaryOperation(_) => {
				let mut binary_operation = expression.binary_operation();
				binary_operation.accept(self)?;

				match binary_operation.operator.node {
					BinaryOperator::Equal => super::application::BOOLEAN_TYPE,
					_ => binary_operation.left.evaluation_type.clone(),
				}
			}
			Expression::FunctionCall(_) => {
				let mut function_call = expression.function_call();
				function_call.accept(self)?;
				function_call.evaluation_type.clone()
			}
		};

		let expression_type = expression.evaluation_type.as_ref();
		Ok(match expression.evaluation_type.resolved().is_some() {
			false => expression.evaluation_type = evaluation_type,
			true => self.unify(expression_type.clone(), evaluation_type.as_ref().clone())
				.map_err(|error| Spanned::new(error.into(), expression.span))?,
		})
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		for parameter in &function.parameters {
			let DataType(parameter_type) = parameter.data_type.clone();
			self.environment.insert(parameter.target.clone(), parameter_type);
		}

		function.statements.iter_mut().try_for_each(|statement| statement.accept(self))?;
		function.return_value.accept(self)?;

		let return_type = function.return_value.evaluation_type.as_ref();
		Ok(self.unify(return_type.clone(), function.return_type.node.as_ref().clone())
			.map_err(|error| Spanned::new(error.into(), function.return_value.span))?)
	}

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|argument| argument.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		Ok(match &mut mutation.node {
			Mutation::Swap(left, right) => {
				let left = self.environment[left].clone();
				let right = self.environment[right].clone();
				self.unify(left, right)
			}
			Mutation::AddAssign(identifier, expression) |
			Mutation::MultiplyAssign(identifier, expression) => {
				expression.accept(self)?;
				let identifier_type = self.environment[identifier].clone();
				let evaluation_type = expression.evaluation_type.as_ref();
				self.unify(identifier_type, evaluation_type.clone())
			}
		}.map_err(|error| Spanned::new(error.into(), mutation.span))?)
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
		for function in syntax_unit.functions.values_mut() {
			if let Err(errors) = function.accept(self) {
				error_collate.combine(errors);
			}

			self.environment.clear();
		}
		error_collate.collapse(())
	}
}
