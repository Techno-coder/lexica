use crate::node::{Arithmetic, BinaryOperator, ExpressionKey, FunctionContext, MutationKind};
use crate::span::{Span, Spanned};

use super::*;
use super::function::basic;

pub fn termination(function: &FunctionContext, context: &mut BasicContext,
                   condition_start: &Option<ExpressionKey>, condition_end: &ExpressionKey,
                   expression: &ExpressionKey, span: Span) -> (Value, Component) {
	let (mut entry, exit) = (context.component(), context.component());
	let (condition_end, end_component) = basic(function, context, condition_end);
	let (_, mut component) = basic(function, context, expression);
	let mut condition_start = condition_start.as_ref().map(|condition_start|
		basic(function, context, condition_start));

	if condition_start.is_none() && context.is_reversible() {
		let variable = context.temporary();
		let compound = Compound::Value(Value::Item(Item::Unsigned64(0)));
		let statement = Statement::Binding(variable.clone(), compound);
		entry = context.push(entry, Spanned::new(statement, span));

		let location = Location::new(variable);
		let value = Value::Item(Item::Unsigned64(1));
		let mutation_kind = MutationKind::Arithmetic(Arithmetic::Add);
		let statement = Statement::Mutation(mutation_kind, location.clone(), value);
		condition_start = Some(comparison_zero(context, location, span));

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

fn comparison_zero(context: &mut BasicContext, location: Location, span: Span) -> (Value, Component) {
	let (variable, value) = (context.temporary(), Value::Item(Item::Unsigned64(0)));
	let comparison = Compound::Binary(BinaryOperator::Equality, Value::Location(location), value);
	let statement = Spanned::new(Statement::Binding(variable.clone(), comparison), span);

	let component = context.component();
	let location = Value::Location(Location::new(variable));
	(location, context.push(component, statement))
}
