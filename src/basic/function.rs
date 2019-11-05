use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{BindingVariable, Expression, ExpressionKey, FunctionContext, MutationKind,
	Parameter, Pattern};
use crate::span::Spanned;

use super::*;

pub fn basic_function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                      reversibility: Reversibility) -> Result<Arc<BasicFunction>, Diagnostic> {
	let context_key = (function_path.node.clone(), reversibility);
	if let Some(function) = context.basic_functions.get(&context_key) {
		return Ok(function.clone());
	}

	let mut parameters = Vec::new();
	let function = crate::node::function(context, function_path)?;
	let span = function.context[&function.expression].span;
	function.function_type.parameters.iter().map(|parameter| &parameter.node)
		.for_each(|Parameter(pattern, _)| pattern.traverse(&mut |terminal| -> Result<_, !> {
			Ok(parameters.push(terminal.clone().map(|BindingVariable(variable, _)| variable)))
		}).unwrap());

	let mut basic_context = BasicContext::new(reversibility);
	let (value, component) = basic(&function.context, &mut basic_context, &function.expression);
	basic_context[&component.exit].advance = Spanned::new(Branch::Return(value), span);

	let (nodes, component) = basic_context.flatten(component);
	let function = Arc::new(BasicFunction { parameters, component, nodes });
	context.basic_functions.insert(context_key, function.clone());
	Ok(function)
}

pub fn basic(function: &FunctionContext, context: &mut BasicContext,
             expression: &ExpressionKey) -> (Value, Component) {
	let expression = &function[expression];
	let span = expression.span;
	match &expression.node {
		Expression::Block(block) => {
			let (mut value, mut component) = (None, context.component());
			for expression in block {
				let (other_value, other) = basic(function, context, expression);
				component = context.join(component, other, span);
				value = Some(other_value);
			}
			(value.unwrap(), component)
		}
		Expression::Binding(binding, _, expression) => {
			let (value, mut component) = basic(function, context, expression);
			match &binding.node {
				Pattern::Wildcard => panic!("Wildcard binding is invalid"),
				Pattern::Terminal(variable) => {
					let BindingVariable(variable, _) = variable.node.clone();
					let statement = Statement::Binding(variable, Compound::Value(value));
					component = context.push(component, Spanned::new(statement, span));
				}
				Pattern::Tuple(_) => {
					let location = match &value {
						Value::Location(location) => location,
						_ => panic!("Tuple binding must be bound to location")
					}.clone();

					component = super::pattern::binding(context,
						component, &value, &binding.node, location);
				}
			}
			(Value::Item(Item::Unit), component)
		}
		Expression::TerminationLoop(condition_start, condition_end, expression) =>
			super::conditional::termination(function, context,
				condition_start, condition_end, expression, span),
		Expression::Conditional(branches) =>
			super::conditional::conditional(function, context, branches, span),
		Expression::Mutation(mutation, mutable, expression) => {
			let (value, component) = basic(function, context, expression);
			let (variable, other) = basic(function, context, mutable);
			let component = context.join(component, other, span);
			// TODO: Implicitly drop location on assignment

			match variable {
				Value::Location(location) => {
					let statement = Statement::Mutation(mutation.node.clone(), location, value);
					(Value::Item(Item::Unit), context.push(component, Spanned::new(statement, span)))
				}
				_ => panic!("Value expression cannot be mutated")
			}
		}
		Expression::ExplicitDrop(_, _) if !context.is_reversible() =>
			(Value::Item(Item::Unit), context.component()),
		Expression::ExplicitDrop(variable, expression) => {
			let (value, component) = basic(function, context, expression);
			let component = context.invert(component);
			let component = match &variable {
				Pattern::Wildcard => panic!("Wildcard explicit drop is invalid"),
				Pattern::Terminal(variable) => {
					let location = Location::new(variable.node.clone());
					let statement = Statement::Mutation(MutationKind::Assign, location, value);

					let mutation = context.component();
					let mutation = context.push(mutation, Spanned::new(statement, span));
					context.join(mutation, component, span)
				}
				Pattern::Tuple(_) => {
					let location = match &value {
						Value::Location(location) => location,
						_ => panic!("Tuple explicit drop must be bound to location")
					}.clone();

					let mutation = context.component();
					let mutation = super::pattern::explicit_drop(context,
						mutation, &value, &variable, location);
					context.join(mutation, component, span)
				}
			};

			let (entry, exit) = (context.component(), context.component());
			context.link(Direction::Advance, &entry, &exit, span);
			context.link(Direction::Advance, &component, &exit, span);
			context.link(Direction::Reverse, &exit, &component, span);
			context.link(Direction::Reverse, &component, &entry, span);
			(Value::Item(Item::Unit), Component::new(entry.entry, exit.exit))
		}
		Expression::Unary(operator, expression) => {
			let variable = context.temporary();
			let (value, component) = basic(function, context, expression);
			let compound = Compound::Unary(operator.node.clone(), value);
			let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
			(Value::Location(Location::new(variable)), context.push(component, statement))
		}
		Expression::Binary(operator, left, right) => {
			let (left_value, left) = basic(function, context, left);
			let (right_value, right) = basic(function, context, right);
			let component = context.join(left, right, span);

			let variable = context.temporary();
			let compound = Compound::Binary(operator.node.clone(), left_value, right_value);
			let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
			(Value::Location(Location::new(variable)), context.push(component, statement))
		}
		Expression::Pattern(expression) => super::pattern::pattern(function, context, expression, span),
		Expression::Variable(variable) => (Value::Location(Location::new(variable.clone())), context.component()),
		// TODO: Resolve integers based on inference type. Pending on trait inference.
		Expression::Unsigned(integer) => (Value::Item(Item::Unsigned64(*integer)), context.component()),
		Expression::Signed(integer) => (Value::Item(Item::Signed64(*integer)), context.component()),
		Expression::Truth(truth) => (Value::Item(Item::Truth(*truth)), context.component()),
	}
}
