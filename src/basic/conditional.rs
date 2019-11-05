use crate::inference::TypeContext;
use crate::node::{Arithmetic, BinaryOperator, ConditionEnd, ConditionStart,
	ExpressionKey, FunctionContext, MutationKind};
use crate::span::{Span, Spanned};

use super::*;
use super::function::basic;

pub fn termination(function: &FunctionContext, context: &mut BasicContext, type_context: &TypeContext,
                   condition_start: &Option<ConditionStart>, condition_end: &ConditionEnd,
                   expression: &ExpressionKey, span: Span) -> (Value, Component) {
	let (mut entry, exit) = (context.component(), context.component());
	let (condition_end, end_component) = basic(function, context, type_context, condition_end);
	let (_, mut component) = basic(function, context, type_context, expression);
	let mut condition_start = condition_start.as_ref().map(|condition_start|
		basic(function, context, type_context, condition_start));

	if condition_start.is_none() && context.is_reversible() {
		let variable = context.temporary();
		let compound = Compound::Value(Value::Item(Item::Unsigned64(0)));
		let statement = Statement::Binding(variable.clone(), compound);
		entry = context.push(entry, Spanned::new(statement, span));

		let location = Location::new(variable);
		let value = Value::Item(Item::Unsigned64(1));
		let mutation_kind = MutationKind::Arithmetic(Arithmetic::Add);
		let statement = Statement::Mutation(mutation_kind, location.clone(), value);
		condition_start = Some(comparison(context, location, span, 0));

		let mut mutation = context.component();
		mutation = context.push(mutation, Spanned::new(statement, span));
		component = context.join(mutation, component, span);
	}

	context.link(Direction::Advance, &entry, &end_component, span);
	context.divergence(Direction::Advance, &end_component,
		Divergence::truth(condition_end, exit.entry, component.entry), span);
	context.link(Direction::Advance, &component, &end_component, span);

	if let Some((condition_start, start_component)) = condition_start {
		let start_component = context.invert(start_component);
		context.link(Direction::Reverse, &exit, &start_component, span);
		context.divergence(Direction::Reverse, &start_component,
			Divergence::truth(condition_start, entry.exit, component.exit), span);
		context.link(Direction::Reverse, &component, &start_component, span);

		context.link(Direction::Reverse, &end_component, &start_component, span);
		context.link(Direction::Advance, &start_component, &end_component, span);
	}

	let component = Component::new(entry.entry, exit.exit);
	(Value::Item(Item::Unit), component)
}

fn comparison(context: &mut BasicContext, location: Location, span: Span, value: u64) -> (Value, Component) {
	let (variable, value) = (context.temporary(), Value::Item(Item::Unsigned64(value)));
	let comparison = Compound::Binary(BinaryOperator::Equality, Value::Location(location), value);
	let statement = Spanned::new(Statement::Binding(variable.clone(), comparison), span);

	let component = context.component();
	let location = Value::Location(Location::new(variable));
	(location, context.push(component, statement))
}

pub fn conditional(function: &FunctionContext, context: &mut BasicContext, type_context: &TypeContext,
                   branches: &[crate::node::Branch], span: Span) -> (Value, Component) {
	let (mut entry, exit) = (context.component(), context.component());
	let mut last_condition_start: Option<Component> = None;
	let mut last_condition_end: Option<Component> = None;
	let all_reversible = branches.iter().map(|(_, condition_end, _)|
		condition_end).all(Option::is_some);

	let mut discriminant = None;
	if !all_reversible {
		let temporary = context.temporary();
		let compound = Compound::Value(Value::Item(Item::Unsigned64(0)));
		let statement = Statement::Binding(temporary.clone(), compound);
		entry = context.push(entry, Spanned::new(statement, span));
		discriminant = Some(Location::new(temporary));
	}

	let mut temporary = None;
	for (index, (condition_start, condition_end, expression)) in branches.iter().enumerate() {
		let expression_span = function[expression].span;
		let condition_start_span = function[condition_start].span;
		let (start_condition, start_component) = basic(function, context, type_context, condition_start);
		let (value, mut component) = basic(function, context, type_context, expression);

		let (end_condition, end_component) = match context.is_reversible() {
			true => match all_reversible {
				true => {
					let condition_end = condition_end.as_ref().unwrap();
					let (end_condition, end_component) = basic(function, context, type_context, condition_end);
					(Some(end_condition), Some((end_component, function[condition_end].span)))
				}
				false => {
					let index = index as u64 + 1;
					let value = Value::Item(Item::Unsigned64(index));
					let statement = Statement::Mutation(MutationKind::Assign,
						discriminant.clone().unwrap(), value);
					component = context.push(component, Spanned::new(statement, span));

					let (end_condition, end_component) = comparison(context,
						discriminant.clone().unwrap(), span, index);
					(Some(end_condition), Some((end_component, span)))
				}
			}
			false => (None, None)
		};

		if value != Value::Item(Item::Unit) {
			let location = temporary.get_or_insert_with(|| Location::new(context.temporary()));
			let statement = Statement::Mutation(MutationKind::Assign, location.clone(), value);
			component = context.push(component, Spanned::new(statement, expression_span));
		}

		let divergence = Divergence::truth(start_condition, component.entry, NodeTarget::UNRESOLVED);
		context.divergence(Direction::Advance, &start_component, divergence, condition_start_span);
		context.link(Direction::Advance, &component, &exit, expression_span);

		if context.is_reversible() {
			let end_condition = end_condition.unwrap();
			let (end_component, condition_end_span) = end_component.as_ref().unwrap();
			let divergence = Divergence::truth(end_condition, component.exit, NodeTarget::UNRESOLVED);
			context.divergence(Direction::Reverse, end_component, divergence, *condition_end_span);
			context.link(Direction::Reverse, &component, &entry, expression_span);
		}

		match last_condition_start {
			Some(last_condition_start) => {
				context.resolve(&last_condition_start.exit, start_component.entry);
				if context.is_reversible() {
					let last_condition_end = last_condition_end.unwrap();
					let (end_component, condition_end_span) = end_component.as_ref().unwrap();
					context.resolve(&last_condition_end.entry, end_component.exit);
					context.link(Direction::Reverse, &start_component,
						&last_condition_start, condition_start_span);
					context.link(Direction::Advance, end_component,
						&last_condition_end, *condition_end_span);
				}
			}
			None => {
				context.link(Direction::Advance, &entry, &start_component, condition_start_span);
				if context.is_reversible() {
					let (end_component, condition_end_span) = end_component.as_ref().unwrap();
					context.link(Direction::Reverse, &start_component, &entry, condition_start_span);
					context.link(Direction::Reverse, &exit, end_component, *condition_end_span);
					context.link(Direction::Advance, end_component, &exit, *condition_end_span);
				}
			}
		}

		last_condition_start = Some(start_component);
		last_condition_end = end_component.map(|(end_component, _)| end_component);
	}

	context.resolve(&last_condition_start.unwrap().exit, exit.entry);
	if let Some(last_condition_end) = last_condition_end {
		context.resolve(&last_condition_end.entry, entry.exit);
		assert!(context.is_reversible());
	}

	let temporary = temporary.map(|temporary| Value::Location(temporary));
	(temporary.unwrap_or(Value::Item(Item::Unit)), Component::new(entry.entry, exit.exit))
}
