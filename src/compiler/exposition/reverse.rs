use crate::interpreter::{Integer, Primitive, Size};
use crate::node::*;
use crate::source::Spanned;

/// Adds reversibility conditions for irreversible constructs.
#[derive(Debug, Default)]
pub struct ReverseExposition<'a> {
	binding_stack: Vec<Spanned<Binding<'a>>>,
	next_temporary: usize,
}

impl<'a> ReverseExposition<'a> {
	pub fn next_target(&mut self) -> VariableTarget<'a> {
		self.next_temporary += 1;
		VariableTarget(Identifier::TEMPORARY_REVERSE, self.next_temporary - 1)
	}

	pub fn pop_bindings(&mut self) -> Vec<Spanned<Statement<'a>>> {
		let mut statements = Vec::new();
		while let Some(binding) = self.binding_stack.pop() {
			let (span, binding) = (binding.span, Statement::Binding(binding));
			statements.push(Spanned::new(binding, span));
		}
		statements
	}
}

impl<'a> NodeVisitor<'a> for ReverseExposition<'a> {
	type Result = ();

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		syntax_unit.structures.values_mut().for_each(|structure| structure.accept(self));
		syntax_unit.functions.values_mut().for_each(|function| function.accept(self));
	}

	fn structure(&mut self, _: &mut Spanned<Structure>) -> Self::Result {}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		self.next_temporary = 0;
		function.expression_block.accept(self);
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		match expression.node.as_mut() {
			Expression::Unit | Expression::Primitive(_) | Expression::Variable(_) => (),
			Expression::BinaryOperation(binary_operation) => binary_operation.accept(self),
			Expression::WhenConditional(when_conditional) => when_conditional.accept(self),
			Expression::ExpressionBlock(expression_block) => expression_block.accept(self),
			Expression::FunctionCall(function_call) => function_call.accept(self),
			Expression::Accessor(accessor) => accessor.accept(self),
		}
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self);
		expression_block.expression.accept(self);
		expression_block.block.statements.append(&mut self.pop_bindings());
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		let mut statements = Vec::new();
		for statement in &mut block.statements {
			statement.accept(self);
			statements.append(&mut self.pop_bindings());
			statements.push(statement.clone());
		}
		block.statements = statements;
	}

	fn accessor(&mut self, accessor: &mut Spanned<Accessor<'a>>) -> Self::Result {
		accessor.expression.accept(self);
		accessor.accessories.iter_mut().for_each(|accessory| match accessory {
			Accessory::FunctionCall(function_call) => function_call.accept(self),
			Accessory::Field(_) => (),
		})
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self);
		operation.right.accept(self);
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self);
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		conditional_loop.block.accept(self);
		if conditional_loop.start_condition.is_none() {
			let (span, size) = (conditional_loop.span, Size::Unsigned64);
			let (target, data_type) = (self.next_target(), size.clone().into());
			let left = Spanned::new(Expression::Variable(target.clone()).into(), span);

			let primitive = Primitive::Integer(Integer::new_unsigned(1));
			let right = Spanned::new(Expression::Primitive(primitive).into(), span);
			let mutation = Mutation::AddAssign(Spanned::new(target.clone(), span), right);
			let statement = Spanned::new(Statement::Mutation(Spanned::new(mutation, span)), span);
			conditional_loop.block.statements.insert(0, statement);

			let right = Spanned::new(Expression::Primitive(size.primitive()).into(), span);
			let right: Spanned<ExpressionNode<'a>> = right;

			let variable = Spanned::new(Variable { target, data_type, is_mutable: true }, span);
			let binding = Binding { variable, expression: right.clone() };
			self.binding_stack.push(Spanned::new(binding, span));

			let operator = Spanned::new(BinaryOperator::Equal, span);
			let operation = Spanned::new(BinaryOperation { left, right, operator }, span);
			let start_condition = Expression::BinaryOperation(operation).into();
			conditional_loop.start_condition = Some(Spanned::new(start_condition, span));
		}
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self);
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		function_call.arguments.iter_mut().for_each(|argument| argument.accept(self));
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		match &mut mutation.node {
			Mutation::Swap(_, _) => {}
			Mutation::Assign(_, expression) |
			Mutation::AddAssign(_, expression) |
			Mutation::MinusAssign(_, expression) |
			Mutation::MultiplyAssign(_, expression) => expression.accept(self),
		}
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		let mut irreversible = false;
		for branch in &mut when_conditional.branches {
			branch.condition.accept(self);
			branch.expression_block.accept(self);
			match &mut branch.end_condition {
				Some(end_condition) => end_condition.accept(self),
				None => irreversible = true,
			}
		}

		if irreversible {
			let span = when_conditional.span;
			let size = match when_conditional.branches.len() {
				length if length < u8::max_value() as usize => Size::Unsigned8,
				length if length < u16::max_value() as usize => Size::Unsigned16,
				length if length < u32::max_value() as usize => Size::Unsigned32,
				length if length < u64::max_value() as usize => Size::Unsigned64,
				_ => panic!("When conditional branch count exceeds limit"),
			};

			let (target, data_type) = (self.next_target(), size.clone().into());
			for (index, branch) in when_conditional.branches.iter_mut().enumerate() {
				let left = Spanned::new(Expression::Variable(target.clone()).into(), span);
				let right = Primitive::Integer(Integer::new_unsigned(index as u64 + 1));
				let right = Spanned::new(Expression::Primitive(right).into(), span);
				let right: Spanned<ExpressionNode<'a>> = right;

				let target = Spanned::new(target.clone(), span);
				let mutation = Spanned::new(Mutation::AddAssign(target, right.clone()), span);
				let statement = Spanned::new(Statement::Mutation(mutation), span);
				branch.expression_block.block.statements.insert(0, statement);

				let operator = Spanned::new(BinaryOperator::Equal, span);
				let operation = Spanned::new(BinaryOperation { left, right, operator }, span);
				let end_condition = Expression::BinaryOperation(operation).into();
				branch.end_condition = Some(Spanned::new(end_condition, span));
			}

			let variable = Spanned::new(Variable { target, data_type, is_mutable: true }, span);
			let expression = Spanned::new(Expression::Primitive(size.primitive()).into(), span);
			self.binding_stack.push(Spanned::new(Binding { variable, expression }, span));
		}
	}
}
