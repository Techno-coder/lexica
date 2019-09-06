use hashbrown::HashMap;
use polytype::{Context, Type, UnificationError};

use crate::interpreter::Primitive;
use crate::node::*;
use crate::source::{ErrorCollate, Spanned};

use super::TypeResult;

/// Applies type inference and analysis.
#[derive(Debug)]
pub struct InferenceEngine<'a> {
	environment: HashMap<VariableTarget<'a>, Type<Identifier<'a>>>,
	context: Context<Identifier<'a>>,
}

impl<'a> InferenceEngine<'a> {
	pub fn context(self) -> Context<Identifier<'a>> {
		self.context
	}

	pub fn unify(&mut self, left: Type<Identifier<'a>>, right: Type<Identifier<'a>>)
	             -> Result<(), UnificationError<Identifier<'a>>> {
		super::application::unify(left, right, &mut self.context)
	}
}

impl<'a> NodeVisitor<'a> for InferenceEngine<'a> {
	type Result = TypeResult<'a, ()>;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		let mut error_collate = ErrorCollate::new();
		apply_unit!(syntax_unit, {
			if let Err(errors) = construct.accept(self) {
				error_collate.combine(errors);
			}

			self.environment.clear();
		}, construct);
		error_collate.collapse(())
	}

	fn structure(&mut self, _: &mut Spanned<Structure<'a>>) -> Self::Result {
		Ok(())
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		for parameter in &function.parameters {
			let DataType(parameter_type) = parameter.data_type.clone();
			self.environment.insert(parameter.target.clone(), parameter_type);
		}

		function.expression_block.accept(self)?;
		let return_value = &function.expression_block.expression;
		let return_type = return_value.evaluation_type.as_ref();
		Ok(self.unify(return_type.clone(), function.return_type.node.as_ref().clone())
			.map_err(|error| Spanned::new(error.into(), return_value.span))?)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let evaluation_type = match expression.node.as_mut() {
			Expression::Unit => DataType::UNIT,
			Expression::Variable(target) => match target.is_root() {
				true => DataType(self.environment[target].clone()),
				false => DataType(self.context.new_variable()),
			},
			Expression::Primitive(primitive) => match primitive {
				Primitive::Boolean(_) => DataType::BOOLEAN,
				_ => DataType(self.context.new_variable()),
			},
			Expression::BinaryOperation(binary_operation) => {
				binary_operation.accept(self)?;
				match binary_operation.operator.node {
					BinaryOperator::Equal => DataType::BOOLEAN,
					_ => binary_operation.left.evaluation_type.clone(),
				}
			}
			Expression::WhenConditional(when_conditional) => {
				when_conditional.accept(self)?;
				when_conditional.branches[0].expression_block
					.expression.evaluation_type.clone()
			}
			Expression::ExpressionBlock(expression_block) => {
				expression_block.accept(self)?;
				expression_block.expression.evaluation_type.clone()
			}
			Expression::FunctionCall(function_call) => {
				function_call.accept(self)?;
				function_call.evaluation_type.clone()
			}
			Expression::Accessor(accessor) => {
				accessor.accept(self)?;
				accessor.evaluation_type.clone()
			}
		};

		let expression_type = expression.evaluation_type.as_ref();
		Ok(match expression.evaluation_type.resolved().is_some() {
			false => expression.evaluation_type = evaluation_type,
			true => self.unify(expression_type.clone(), evaluation_type.as_ref().clone())
				.map_err(|error| Spanned::new(error.into(), expression.span))?,
		})
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self)?;
		expression_block.expression.accept(self)
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		block.statements.iter_mut().try_for_each(|statement| statement.accept(self))
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result where Self: Sized {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
			Statement::Expression(expression) => {
				expression.accept(self)?;
				let evaluation_type = expression.evaluation_type.as_ref().clone();
				Ok(self.unify(evaluation_type, DataType::UNIT.as_ref().clone())
					.map_err(|error| Spanned::new(error.into(), statement.span))?)
			}
		}
	}

	fn accessor(&mut self, accessor: &mut Spanned<Accessor<'a>>) -> Self::Result {
		accessor.evaluation_type = DataType(self.context.new_variable());
		accessor.function_call.iter_mut().try_for_each(|function_call| function_call.accept(self))?;
		accessor.expression.accept(self)
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
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

		self.unify(start_condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
			.map_err(|error| Spanned::new(error.into(), start_condition.span))?;
		self.unify(conditional_loop.end_condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
			.map_err(|error| Spanned::new(error.into(), conditional_loop.end_condition.span))?;

		conditional_loop.block.accept(self)
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self)?;
		Ok(match explicit_drop.target.is_root() {
			true => {
				let identifier_type = self.environment[&explicit_drop.target].clone();
				let expression_type = explicit_drop.expression.evaluation_type.as_ref();
				self.unify(identifier_type, expression_type.clone())
					.map_err(|error| Spanned::new(error.into(), explicit_drop.span))?
			}
			false => (),
		})
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|argument| argument.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		Ok(match &mut mutation.node {
			Mutation::Swap(left, right) => {
				match left.is_root() && right.is_root() {
					true => {
						let left = self.environment[left].clone();
						let right = self.environment[right].clone();
						self.unify(left, right)
					}
					false => return Ok(()),
				}
			}
			Mutation::Assign(identifier, expression) |
			Mutation::AddAssign(identifier, expression) |
			Mutation::MinusAssign(identifier, expression) |
			Mutation::MultiplyAssign(identifier, expression) => {
				expression.accept(self)?;
				match identifier.is_root() {
					true => {
						let identifier_type = self.environment[identifier].clone();
						let evaluation_type = expression.evaluation_type.as_ref();
						self.unify(identifier_type, evaluation_type.clone())
					}
					false => return Ok(()),
				}
			}
		}.map_err(|error| Spanned::new(error.into(), mutation.span))?)
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		for branch in &mut when_conditional.branches {
			branch.condition.accept(self)?;
			self.unify(branch.condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
				.map_err(|error| Spanned::new(error.into(), branch.condition.span))?;

			let end_condition = branch.end_condition.as_mut().unwrap();
			end_condition.accept(self)?;

			self.unify(end_condition.evaluation_type.as_ref().clone(), BOOLEAN_TYPE)
				.map_err(|error| Spanned::new(error.into(), end_condition.span))?;
			branch.expression_block.accept(self)?;
		}

		for (right_index, right) in when_conditional.branches.iter().enumerate().skip(1) {
			let left = &when_conditional.branches[right_index - 1];
			let span = right.expression_block.expression.span;

			let left = left.expression_block.expression.evaluation_type.as_ref().clone();
			let right = right.expression_block.expression.evaluation_type.as_ref().clone();
			self.unify(left, right).map_err(|error| Spanned::new(error.into(), span))?;
		}

		Ok(())
	}
}

impl<'a> Default for InferenceEngine<'a> {
	fn default() -> Self {
		let mut context = Context::default();
		let _ = context.new_variable();
		Self { environment: HashMap::new(), context }
	}
}
