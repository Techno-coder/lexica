use hashbrown::HashMap;
use polytype::{Context, Type};

use crate::node::*;
use crate::source::{ErrorCollate, Span, Spanned};

use super::{TypeError, TypeResult};

/// Resolves evaluation types on accessors.
/// Verifies accessories on structure exists.
#[derive(Debug)]
pub struct AccessResolver<'a> {
	environment: HashMap<VariableTarget<'a>, Type<Identifier<'a>>>,
	structures: StructureMap<'a>,
	context: Context<Identifier<'a>>,
}

impl<'a> AccessResolver<'a> {
	pub fn new(context: Context<Identifier<'a>>) -> Self {
		Self { environment: HashMap::new(), structures: HashMap::new(), context }
	}

	pub fn get_structure(&self, identifier: &Identifier<'a>, span: Span)
	                     -> TypeResult<'a, &Spanned<Structure<'a>>> {
		let error = TypeError::UndefinedStructure(identifier.clone());
		self.structures.get(identifier).ok_or(Spanned::new(error, span).into())
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

			self.environment.clear();
		}, construct);
		error_collate.collapse(())
	}

	fn structure(&mut self, _: &mut Spanned<Structure>) -> Self::Result {
		Ok(())
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		for parameter in &function.parameters {
			super::application::defined(&self.structures, &parameter.data_type, parameter.span)?;
			self.environment.insert(parameter.target.clone(), parameter.data_type.as_ref().clone());
		}
		function.expression_block.accept(self)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		match expression.node.as_mut() {
			Expression::Variable(variable) => {
				let mut data_type = self.environment[&variable.root()].clone();
				super::application::apply(&self.context, &mut data_type);

				let error = TypeError::UnresolvedType(data_type.clone());
				let mut identifier = Identifier(DataType(data_type).resolved()
					.ok_or(Spanned::new(error, expression.span))?);

				let VariableTarget(_, _, accessor) = variable;
				for accessory in accessor.split() {
					let structure = self.get_structure(&identifier, expression.span)?;
					let error = TypeError::UndefinedAccessory(structure.identifier.node.clone(), accessory.clone());
					let field = structure.fields.get(&accessory).ok_or(Spanned::new(error, expression.span))?;
					identifier = Identifier(field.data_type.resolved().unwrap());
				}

				let DataType(data_type) = DataType::new(identifier);
				let evaluation_type = expression.evaluation_type.as_ref().clone();
				Ok(super::application::unify(evaluation_type, data_type, &mut self.context)
					.map_err(|error| Spanned::new(error.into(), expression.span))?)
			}
			Expression::Unit | Expression::Primitive(_) => Ok(()),
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

	// TODO: Evaluate AccessorCall
	fn accessor(&mut self, accessor: &mut Spanned<Accessor<'a>>) -> Self::Result {
		accessor.expression.accept(self)?;
		let expression_type = accessor.expression.evaluation_type.clone();
		let error = TypeError::UnresolvedType(expression_type.as_ref().clone());
		let error = Spanned::new(error, accessor.expression.span);

		let identifier = Identifier(expression_type.resolved().ok_or(error)?);
		let mut identifier = Spanned::new(identifier, accessor.span);

		for accessory in &mut accessor.accessories {
			let structure = self.get_structure(&identifier, identifier.span)?;
			identifier = match accessory {
				Accessory::FunctionCall(function_call) => {
					function_call.accept(self)?;
					// TODO: Check method exists
					unimplemented!()
				}
				Accessory::Field(field) => {
					let error = TypeError::UndefinedAccessory(identifier.node.clone(), field.node.clone());
					let field = structure.fields.get(&field.node).ok_or(Spanned::new(error, field.span))?;
					let data_type = Identifier(field.data_type.resolved().unwrap());
					Spanned::new(data_type, field.data_type.span)
				}
			}
		}

		let DataType(data_type) = DataType::new(identifier.node);
		let evaluation_type = accessor.evaluation_type.as_ref().clone();
		Ok(super::application::unify(evaluation_type, data_type, &mut self.context)
			.map_err(|error| Spanned::new(error.into(), accessor.span))?)
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self)?;
		operation.right.accept(self)?;
		Ok(())
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self)?;
		let binding_type = binding.variable.data_type.as_ref().clone();
		self.environment.insert(binding.variable.target.clone(), binding_type);
		Ok(())
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
