use hashbrown::HashMap;
use polytype::Context;

use crate::node::*;
use crate::source::{ErrorCollate, Spanned};

use super::{TypeError, TypeResult};

/// Resolves evaluation types on accessors.
/// Verifies accessories on structure exists.
#[derive(Debug)]
pub struct AccessResolver<'a> {
	structures: HashMap<Identifier<'a>, Spanned<Structure<'a>>>,
	context: Context<Identifier<'a>>,
}

impl<'a> AccessResolver<'a> {
	pub fn new(context: Context<Identifier<'a>>) -> Self {
		Self { structures: HashMap::new(), context }
	}

	pub fn get_structure(&self, identifier: &Spanned<Identifier<'a>>)
	                     -> TypeResult<'a, &Spanned<Structure<'a>>> {
		let error = TypeError::UndefinedStructure(identifier.node.clone());
		self.structures.get(&identifier).ok_or(Spanned::new(error, identifier.span).into())
	}

	pub fn context(self) -> Context<Identifier<'a>> {
		self.context
	}
}

impl<'a> NodeVisitor<'a> for AccessResolver<'a> {
	type Result = TypeResult<'a, ()>;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		let mut error_collate = ErrorCollate::new();
		self.structures = syntax_unit.structures.clone();
		apply_unit!(syntax_unit, {
			if let Err(errors) = construct.accept(self) {
				error_collate.combine(errors);
			}
		}, construct);
		error_collate.collapse(())
	}

	fn structure(&mut self, _: &mut Spanned<Structure>) -> Self::Result {
		Ok(())
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.expression_block.accept(self)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		match expression.node.as_mut() {
			Expression::Unit | Expression::Variable(_) | Expression::Primitive(_) => Ok(()),
			Expression::BinaryOperation(binary_operation) => binary_operation.accept(self),
			Expression::WhenConditional(when_conditional) => when_conditional.accept(self),
			Expression::ExpressionBlock(expression_block) => expression_block.accept(self),
			Expression::FunctionCall(function_call) => function_call.accept(self),
			Expression::Accessor(accessor) => accessor.accept(self),
		}
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self)?;
		expression_block.expression.accept(self)
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		block.statements.iter_mut().try_for_each(|statement| statement.accept(self))
	}

	fn accessor(&mut self, accessor: &mut Spanned<Accessor<'a>>) -> Self::Result {
		accessor.expression.accept(self)?;
		let evaluation_type = accessor.expression.evaluation_type.clone();
		let error = TypeError::UnresolvedType(evaluation_type.as_ref().clone());
		let error = Spanned::new(error, accessor.expression.span);

		let identifier = Identifier(evaluation_type.resolved().ok_or(error)?);
		let mut identifier = Spanned::new(identifier, accessor.span);

		for accessory in &mut accessor.accessories {
			let structure = self.get_structure(&identifier)?;
			identifier = match accessory {
				Accessory::FunctionCall(function_call) => {
					function_call.accept(self)?;
					// TODO: Check method exists
					unimplemented!()
				}
				Accessory::Field(field) => {
					let error = TypeError::UndefinedAccessory(identifier.node.clone(), field.node.clone());
					structure.fields.get(&field.node).ok_or(Spanned::new(error, field.span))?
						.identifier.clone()
				}
			}
		}

		let DataType(data_type) = DataType::new(identifier.node);
		let evaluation_type = evaluation_type.as_ref().clone();
		Ok(super::application::unify(evaluation_type, data_type, &mut self.context)
			.map_err(|error| Spanned::new(error.into(), accessor.span))?)
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self)?;
		operation.right.accept(self)?;
		Ok(())
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self)
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.block.accept(self)?;
		conditional_loop.start_condition.as_mut().unwrap().accept(self)?;
		conditional_loop.end_condition.accept(self)
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self)
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|expression| expression.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(_, _) => Ok(()),
			Mutation::Assign(_, expression) |
			Mutation::AddAssign(_, expression) |
			Mutation::MinusAssign(_, expression) |
			Mutation::MultiplyAssign(_, expression) => expression.accept(self),
		}
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		Ok(for branch in &mut when_conditional.branches {
			branch.condition.accept(self)?;
			branch.end_condition.as_mut().unwrap().accept(self)?;
			branch.expression_block.accept(self)?;
		})
	}
}
