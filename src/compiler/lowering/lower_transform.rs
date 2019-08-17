use hashbrown::HashMap;

use crate::basic;
use crate::node::*;
use crate::source::Spanned;

use super::Component;

#[derive(Debug, Default)]
pub struct LowerTransform<'a> {
	functions: HashMap<Identifier<'a>, basic::Function<'a>>,
	bindings: HashMap<VariableTarget<'a>, Variable<'a>>,

	evaluation_stack: Vec<basic::Value<'a>>,
	component_stack: Vec<Component<'a>>,
	next_temporary: usize,
	next_block: usize,
}

impl<'a> LowerTransform<'a> {
	pub fn next_temporary(&mut self) -> VariableTarget<'a> {
		self.next_temporary += 1;
		VariableTarget(Identifier::TEMPORARY, self.next_temporary - 1)
	}

	pub fn next_block(&mut self) -> basic::BlockTarget {
		self.next_block += 1;
		basic::BlockTarget(self.next_block - 1)
	}

	pub fn pop_evaluation(&mut self) -> basic::Value<'a> {
		self.evaluation_stack.pop().expect("Evaluation stack is empty")
	}

	pub fn pop_expression(&mut self) -> (Spanned<basic::Expression<'a>>, Component<'a>) {
		let value = self.pop_evaluation();
		if let basic::Value::Expression(expression) = value {
			return (expression, Component::new_empty(self.next_block()));
		}

		let span = value.span();
		let (target, data_type) = (self.next_temporary(), value.data_type());
		let binding_variable = Variable { target, data_type, is_mutable: false };
		let variable = Spanned::new(binding_variable.clone(), span);

		let binding = Spanned::new(basic::Binding { variable, value }, span);
		let statement = Spanned::new(basic::Statement::Binding(binding), span);
		let block = basic::BasicBlock::new_single(statement);

		(Spanned::new(basic::Expression::Variable(binding_variable), span),
			Component::new_single(self.next_block(), block))
	}

	pub fn pop_component(&mut self) -> Component<'a> {
		self.component_stack.pop().expect("Component stack is empty")
	}
}

impl<'a> NodeVisitor<'a> for LowerTransform<'a> {
	type Result = ();

	fn syntax_unit(&mut self, syntax_unit: &mut Spanned<SyntaxUnit<'a>>) -> Self::Result {
		syntax_unit.functions.values_mut().for_each(|function| function.accept(self));
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		self.bindings.clear();
		self.next_temporary = 0;
		self.next_block = 0;

		for parameter in &function.parameters {
			let target = parameter.target.clone();
			let parameter = parameter.node.clone();
			self.bindings.insert(target, parameter);
		}

		function.expression_block.accept(self);
		let component = self.pop_component();
		let (expression, other) = self.pop_expression();
		let mut component = component.join(other);

		let reverse_branch = Spanned::new(basic::Expression::Unit, function.span);
		component.blocks.get_mut(&component.reverse_block).unwrap()
			.reverse = basic::Branch::Return(reverse_branch);
		component.blocks.get_mut(&component.advance_block).unwrap()
			.advance = basic::Branch::Return(expression);

		let identifier = function.identifier.node.clone();
		let function = component.compress_function();
		println!("{}", function); // TODO
		self.functions.insert(identifier, function);

		assert!(self.evaluation_stack.is_empty());
		assert!(self.component_stack.is_empty());
	}

	fn expression(&mut self, expression: &mut Spanned<ExpressionNode<'a>>) -> Self::Result {
		let expression_span = expression.span;
		let expression = Spanned::new(match expression.node.as_mut() {
			Expression::Unit => basic::Expression::Unit,
			Expression::Primitive(primitive) => basic::Expression::Primitive(primitive.clone()),
			Expression::Variable(target) => {
				let error = format!("Binding for: {}, does not exist", target);
				let binding = self.bindings.get(&target).expect(&error);
				basic::Expression::Variable(binding.clone())
			}
			Expression::BinaryOperation(binary_operation) => return binary_operation.accept(self),
			Expression::WhenConditional(when_conditional) => return when_conditional.accept(self),
			Expression::ExpressionBlock(expression_block) => return expression_block.accept(self),
			Expression::FunctionCall(function_call) => return function_call.accept(self),
		}, expression_span);

		let next_block = self.next_block();
		self.component_stack.push(Component::new_empty(next_block));
		self.evaluation_stack.push(basic::Value::Expression(expression));
	}

	fn expression_block(&mut self, expression_block: &mut Spanned<ExpressionBlock<'a>>) -> Self::Result {
		expression_block.block.accept(self);
		let component = self.pop_component();
		expression_block.expression.accept(self);

		let component = component.join(self.pop_component());
		self.component_stack.push(component);
	}

	fn block(&mut self, block: &mut Spanned<Block<'a>>) -> Self::Result {
		let mut component = Component::new_empty(self.next_block());
		for statement in &mut block.statements {
			statement.accept(self);
			component = component.join(self.pop_component());
		}
		self.component_stack.push(component);
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result where Self: Sized {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
			Statement::Expression(expression) => {
				expression.accept(self);
				if let basic::Value::FunctionCall(function_call) = self.pop_evaluation() {
					let (span, component) = (function_call.span, self.pop_component());
					let statement = basic::Statement::FunctionCall(function_call);
					let statement = Spanned::new(statement, span);

					let component = component.append(self.next_block(), statement);
					self.component_stack.push(component);
				}
			}
		}
	}

	fn binary_operation(&mut self, operation: &mut Spanned<BinaryOperation<'a>>) -> Self::Result {
		operation.left.accept(self);
		let (left, other) = self.pop_expression();
		let component = self.pop_component().join(other);

		operation.right.accept(self);
		let (right, other) = self.pop_expression();
		let other = self.pop_component().join(other);
		let component = component.join(other);
		self.component_stack.push(component);

		let (span, operator) = (operation.span, operation.operator.clone());
		let operation = basic::BinaryOperation { left, right, operator };
		let operation = Spanned::new(operation, span);

		let value = basic::Value::BinaryOperation(operation);
		self.evaluation_stack.push(value);
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		binding.expression.accept(self);
		let component = self.pop_component();

		let (span, variable) = (binding.span, binding.variable.clone());
		self.bindings.insert(variable.target.clone(), variable.node.clone());
		let binding = basic::Binding { variable, value: self.pop_evaluation() };
		let binding = Spanned::new(binding, span);

		let statement = Spanned::new(basic::Statement::Binding(binding), span);
		let component = component.append(self.next_block(), statement);
		self.component_stack.push(component);
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		let (entry_target, exit_target) = (self.next_block(), self.next_block());
		let mut component = Component::new_paired(entry_target.clone(), exit_target.clone());

		conditional_loop.start_condition.as_mut().unwrap().accept(self);
		let (start_condition, other) = self.pop_expression();
		let mut start_component = self.pop_component().join(other);
		component.incorporate(&mut start_component);

		conditional_loop.end_condition.accept(self);
		let (end_condition, other) = self.pop_expression();
		let mut end_component = self.pop_component().join(other).invert();
		component.incorporate(&mut end_component);

		conditional_loop.block.accept(self);
		let mut block_component = self.pop_component();
		component.incorporate(&mut block_component);

		component.link_advance(&entry_target, &end_component.reverse_block);
		component.link_advance(&block_component.advance_block, &end_component.reverse_block);
		component.link_reverse(&end_component.reverse_block, &start_component.advance_block);
		component.conditional_advance(&end_component.advance_block, basic::ConditionalBranch {
			condition: end_condition,
			target: exit_target.clone(),
			default: block_component.reverse_block.clone(),
		});

		component.link_reverse(&exit_target, &start_component.advance_block);
		component.link_reverse(&block_component.reverse_block, &start_component.advance_block);
		component.link_advance(&start_component.advance_block, &end_component.reverse_block);
		component.conditional_reverse(&start_component.reverse_block, basic::ConditionalBranch {
			condition: start_condition,
			target: entry_target.clone(),
			default: block_component.advance_block.clone(),
		});

		self.component_stack.push(component);
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		explicit_drop.expression.accept(self);
		let target = explicit_drop.target.clone();
		let (expression, other) = self.pop_expression();
		let component = self.pop_component().join(other);

		let span = explicit_drop.span;
		let explicit_drop = Spanned::new(basic::ExplicitDrop { target, expression }, span);
		let statement = Spanned::new(basic::Statement::ExplicitDrop(explicit_drop), span);
		let mut component = component.append(self.next_block(), statement).invert();

		let (entry_target, exit_target) = (self.next_block(), self.next_block());
		component.blocks.insert(entry_target.clone(), basic::BasicBlock::default());
		component.blocks.insert(exit_target.clone(), basic::BasicBlock::default());

		let advance_block = component.advance_block.clone();
		component.link_advance(&entry_target, &exit_target);
		component.link_advance(&advance_block, &exit_target);
		component.link_reverse(&exit_target, &advance_block);
		component.link_reverse(&advance_block, &entry_target);

		component.advance_block = exit_target.clone();
		component.reverse_block = entry_target.clone();
		self.component_stack.push(component);
	}

	fn function_call(&mut self, function_call: &mut Spanned<FunctionCall<'a>>) -> Self::Result {
		let mut arguments = Vec::new();
		let mut component = Component::new_empty(self.next_block());
		for argument in &mut function_call.arguments {
			argument.accept(self);
			let (expression, other) = self.pop_expression();
			let other = self.pop_component().join(other);
			component = component.join(other);
			arguments.push(expression);
		}

		let span = function_call.span;
		let function = function_call.function.clone();
		let evaluation_type = function_call.evaluation_type.clone();
		let function_call = basic::FunctionCall { function, arguments, evaluation_type };

		let function_call = Spanned::new(function_call, span);
		let value = basic::Value::FunctionCall(function_call);
		self.component_stack.push(component);
		self.evaluation_stack.push(value);
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		let span = mutation.span;
		let expression = match &mut mutation.node {
			Mutation::Swap(left, right) => {
				let mutation = basic::Mutation::Swap(left.clone(), right.clone());
				let statement = basic::Statement::Mutation(Spanned::new(mutation, span));
				let block = basic::BasicBlock::new_single(Spanned::new(statement, span));
				let component = Component::new_single(self.next_block(), block);
				return self.component_stack.push(component);
			}
			Mutation::AddAssign(_, expression) => expression,
			Mutation::MinusAssign(_, expression) => expression,
			Mutation::MultiplyAssign(_, expression) => expression,
		};

		expression.accept(self);
		let (expression, other) = self.pop_expression();
		let mutation = Spanned::new(match &mut mutation.node {
			Mutation::Swap(_, _) => unreachable!(),
			Mutation::AddAssign(target, _) =>
				basic::Mutation::AddAssign(target.clone(), expression),
			Mutation::MinusAssign(target, _) =>
				basic::Mutation::MinusAssign(target.clone(), expression),
			Mutation::MultiplyAssign(target, _) =>
				basic::Mutation::MultiplyAssign(target.clone(), expression),
		}, span);

		let statement = Spanned::new(basic::Statement::Mutation(mutation), span);
		let component = self.pop_component().join(other).append(self.next_block(), statement);
		self.component_stack.push(component);
	}

	fn when_conditional(&mut self, when_conditional: &mut Spanned<WhenConditional<'a>>) -> Self::Result {
		let (entry_target, exit_target) = (self.next_block(), self.next_block());
		let mut component = Component::new_paired(entry_target.clone(), exit_target.clone());
		let mut temporary: Option<Variable<'a>> = None;

		let mut last_condition: Option<basic::BlockTarget> = None;
		let mut last_end_condition: Option<basic::BlockTarget> = None;
		for branch in when_conditional.branches.iter_mut() {
			branch.condition.accept(self);
			let (condition, other) = self.pop_expression();
			let mut condition_component = self.pop_component().join(other);
			component.incorporate(&mut condition_component);

			branch.end_condition.as_mut().unwrap().accept(self);
			let (end_condition, other) = self.pop_expression();
			let mut end_condition_component = self.pop_component().join(other).invert();
			component.incorporate(&mut end_condition_component);

			branch.expression_block.accept(self);
			let (expression, other) = self.pop_expression();
			let mut block_component = self.pop_component().join(other);

			let data_type = expression.data_type();
			if data_type != DataType::UNIT {
				let expression_span = expression.span;
				let variable = temporary.get_or_insert_with(|| {
					let target = self.next_temporary();
					Variable { target, data_type, is_mutable: false }
				});

				let target = Spanned::new(variable.target.clone(), expression_span);
				let assignment = basic::Assignment { target, expression };
				let assignment = Spanned::new(assignment, expression_span);
				let statement = basic::Statement::Assignment(assignment);
				let statement = Spanned::new(statement, expression_span);
				block_component = block_component.append(self.next_block(), statement);
			}

			component.incorporate(&mut block_component);
			component.link_advance(&block_component.advance_block, &exit_target);
			component.link_reverse(&block_component.reverse_block, &entry_target);

			let (target, default) = (block_component.reverse_block, basic::BlockTarget::SENTINEL);
			component[&target].in_advance.push(condition_component.advance_block.clone());
			let branch = basic::ConditionalBranch { condition, target, default };
			let branch = basic::Branch::Conditional(branch);
			component[&condition_component.advance_block].advance = branch;

			let (target, default) = (block_component.advance_block, basic::BlockTarget::SENTINEL);
			component[&target].in_reverse.push(end_condition_component.reverse_block.clone());
			let branch = basic::ConditionalBranch { condition: end_condition, target, default };
			let branch = basic::Branch::Conditional(branch);
			component[&end_condition_component.reverse_block].reverse = branch;

			component.link_reverse(&condition_component.reverse_block,
				last_condition.as_ref().unwrap_or(&entry_target));
			component.link_advance(&end_condition_component.advance_block,
				last_end_condition.as_ref().unwrap_or(&exit_target));

			match &last_condition {
				Some(last_condition) => {
					let default_target = condition_component.reverse_block;
					component[&default_target].in_advance.push(last_condition.clone());
					let mapping = map! { basic::BlockTarget::SENTINEL => default_target };
					component[last_condition].advance.replace(&mapping);

					let last_end_condition = last_end_condition.as_ref().unwrap();
					let default_target = end_condition_component.advance_block;
					component[&default_target].in_reverse.push(last_end_condition.clone());
					let mapping = map! { basic::BlockTarget::SENTINEL => default_target };
					component[last_end_condition].reverse.replace(&mapping);
				}
				None => {
					component.link_advance(&entry_target, &condition_component.reverse_block);
					component.link_reverse(&exit_target, &end_condition_component.advance_block);
				}
			}

			last_condition = Some(condition_component.advance_block);
			last_end_condition = Some(end_condition_component.reverse_block);
		}

		let mapping = map! { basic::BlockTarget::SENTINEL => exit_target };
		component[&last_condition.unwrap()].advance.replace(&mapping);
		let mapping = map! { basic::BlockTarget::SENTINEL => entry_target.clone() };
		component[&last_end_condition.unwrap()].reverse.replace(&mapping);

		let span = when_conditional.span;
		let expression = Spanned::new(match temporary {
			Some(temporary) => {
				let variable = Spanned::new(temporary.clone(), span);
				let value = basic::Value::Uninitialized(span);
				let binding = basic::Binding { variable, value };
				let binding = Spanned::new(binding, span);

				let statement = basic::Statement::Binding(binding);
				let statement = Spanned::new(statement, span);
				component[&entry_target].statements.push(statement);
				basic::Expression::Variable(temporary)
			}
			None => basic::Expression::Unit,
		}, span);

		let value = basic::Value::Expression(expression);
		self.component_stack.push(component);
		self.evaluation_stack.push(value);
	}
}
