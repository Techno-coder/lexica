use crate::basic;
use crate::node::{Binding, ExplicitDrop, Mutation, NodeConstruct, Statement};
use crate::source::Spanned;

use super::{Component, LowerTransform};

pub fn statement<'a>(transform: &mut LowerTransform<'a>, statement: &mut Spanned<Statement<'a>>) {
	match &mut statement.node {
		Statement::Binding(binding) => binding.accept(transform),
		Statement::Mutation(mutation) => mutation.accept(transform),
		Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(transform),
		Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(transform),
		Statement::Expression(expression) => {
			expression.accept(transform);
			if let basic::Value::FunctionCall(function_call) = transform.pop_evaluation() {
				let (span, component) = (function_call.span, transform.pop_component());
				let statement = basic::Statement::FunctionCall(function_call);
				let statement = Spanned::new(statement, span);

				let component = component.append(transform.next_block(), statement);
				transform.push_component(component);
			}
		}
	}
}

pub fn binding<'a>(transform: &mut LowerTransform<'a>, binding: &mut Spanned<Binding<'a>>) {
	binding.expression.accept(transform);
	let component = transform.pop_component();

	let span = binding.span;
	let variable = binding.variable.clone();
	transform.bind_variable(variable.node.clone());
	let binding = basic::Binding { variable, value: transform.pop_evaluation() };
	let binding = Spanned::new(binding, span);

	let statement = Spanned::new(basic::Statement::Binding(binding), span);
	let component = component.append(transform.next_block(), statement);
	transform.push_component(component);
}

pub fn explicit_drop<'a>(transform: &mut LowerTransform<'a>, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) {
	explicit_drop.expression.accept(transform);
	let target = explicit_drop.target.clone();
	let (expression, other) = transform.pop_expression();
	let component = transform.pop_component().join(other, expression.span);

	let span = explicit_drop.span;
	let assignment = Spanned::new(basic::Assignment { target, expression }, span);
	let statement = Spanned::new(basic::Statement::Assignment(assignment), span);
	let mut component = component.append(transform.next_block(), statement).invert();

	let (entry_target, exit_target) = (transform.next_block(), transform.next_block());
	component.blocks.insert(entry_target.clone(), basic::BasicBlock::default());
	component.blocks.insert(exit_target.clone(), basic::BasicBlock::default());

	let advance_block = component.advance_block.clone();
	component.link_advance(&entry_target, &exit_target, span);
	component.link_advance(&advance_block, &exit_target, span);
	component.link_reverse(&exit_target, &advance_block, span);
	component.link_reverse(&advance_block, &entry_target, span);

	component.advance_block = exit_target.clone();
	component.reverse_block = entry_target.clone();
	transform.push_component(component);
}

pub fn mutation<'a>(transform: &mut LowerTransform<'a>, mutation: &mut Spanned<Mutation<'a>>) {
	let span = mutation.span;
	let expression = match &mut mutation.node {
		Mutation::Swap(left, right) => {
			let mutation = basic::Mutation::Swap(left.clone(), right.clone());
			let statement = basic::Statement::Mutation(Spanned::new(mutation, span));
			let block = basic::BasicBlock::new_single(Spanned::new(statement, span));
			let component = Component::new_single(transform.next_block(), block);
			return transform.push_component(component);
		}
		Mutation::AddAssign(_, expression) => expression,
		Mutation::MinusAssign(_, expression) => expression,
		Mutation::MultiplyAssign(_, expression) => expression,
	};

	expression.accept(transform);
	let (expression, other) = transform.pop_expression();
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
	let component = transform.pop_component().join(other, span)
		.append(transform.next_block(), statement);
	transform.push_component(component);
}
