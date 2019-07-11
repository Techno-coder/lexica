use polytype::Context;

use crate::interpreter::Size;
use crate::node::*;
use crate::source::{ErrorCollate, Span, Spanned};

use super::{TypeError, TypeResult};

/// Applies resolved types from a type context.
/// Checks for type equality within nodes that cannot be checked at unification.
#[derive(Debug)]
pub struct TypeAnnotator<'a> {
	context: Context<Identifier<'a>>,
}

impl<'a> TypeAnnotator<'a> {
	pub fn new(context: Context<Identifier<'a>>) -> Self {
		Self { context }
	}

	pub fn apply(&mut self, data_type: &mut DataType<'a>, span: Span) -> TypeResult<'a, ()> {
		let DataType(internal_type) = data_type;
		assert_ne!(internal_type, &TYPE_SENTINEL);

		super::application::apply(&self.context, internal_type);
		let type_error = TypeError::UnresolvedType(internal_type.clone());
		data_type.resolved().ok_or(Spanned::new(type_error, span))?;
		Ok(())
	}
}

impl<'a> NodeVisitor<'a> for TypeAnnotator<'a> {
	type Result = TypeResult<'a, ()>;

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		let mut error_collate = ErrorCollate::new();
		for function in syntax_unit.functions.values_mut() {
			if let Err(errors) = function.accept(self) {
				error_collate.combine(errors);
			}
		}
		error_collate.collapse(())
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		function.expression_block.accept(self)
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let expression_span = expression.span;
		self.apply(&mut expression.evaluation_type, expression_span)?;
		let evaluation_type = expression.evaluation_type.as_ref().clone();

		let type_identifier = expression.evaluation_type.resolved().unwrap();
		match &mut expression.expression {
			Expression::Primitive(primitive) => {
				let error = TypeError::PrimitiveConflict(primitive.clone(), evaluation_type);
				let error = Spanned::new(error, expression_span);
				let size = Size::parse(type_identifier).map_err(|_| error.clone())?;
				Ok(*primitive = primitive.clone().cast(size).ok_or(error)?)
			}
			Expression::BinaryOperation(_) => expression.binary_operation().accept(self),
			Expression::WhenConditional(_) => expression.when_conditional().accept(self),
			Expression::FunctionCall(_) => expression.function_call().accept(self),
			_ => Ok(()),
		}
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self)?;
		expression_block.expression.accept(self)
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		block.statements.iter_mut().try_for_each(|statement| statement.accept(self))
	}

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self)?;
		operation.right.accept(self)?;
		Ok(())
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let variable_span = binding.variable.span.clone();
		self.apply(&mut binding.variable.data_type, variable_span)?;
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

	fn function_call(&mut self, function_call: &mut Spanned<&mut FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().try_for_each(|expression| expression.accept(self))
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::AddAssign(_, expression) |
			Mutation::MultiplyAssign(_, expression) => expression.accept(self),
			_ => Ok(()),
		}
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<&mut WhenConditional<'a>>) -> Self::Result {
		Ok(for branch in &mut when_conditional.branches {
			branch.condition.accept(self)?;
			branch.end_condition.as_mut().unwrap().accept(self)?;
			branch.expression_block.accept(self)?;
		})
	}
}
