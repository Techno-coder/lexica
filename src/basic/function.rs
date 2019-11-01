use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{BindingVariable, Expression, ExpressionKey, FunctionContext, Parameter, Pattern};
use crate::span::Spanned;

use super::*;

pub fn basic_function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>)
                      -> Result<Arc<BasicFunction>, Diagnostic> {
	if let Some(function) = context.basic_functions.get(&function_path.node) {
		return Ok(function.clone());
	}

	let mut basic_context = BasicContext::default();
	let function = crate::node::function(context, function_path)?;
	let (value, component) = basic(&function.context, &mut basic_context, &function.expression);

	let span = function.context[&function.expression].span;
	basic_context[&component.exit].advance = Spanned::new(Branch::Return(value), span);

	let mut parameters = Vec::new();
	function.function_type.parameters.iter().map(|parameter| &parameter.node)
		.for_each(|Parameter(pattern, _)| pattern.traverse(&mut |terminal| -> Result<_, !> {
			Ok(parameters.push(terminal.clone().map(|BindingVariable(variable, _)| variable)))
		}).unwrap());

	let nodes = basic_context.flatten();
	let function = Arc::new(BasicFunction { parameters, component, nodes });
	context.basic_functions.insert(function_path.node.clone(), function.clone());
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

					component = super::pattern::binding(context, component,
						&value, &binding.node, location);
				}
			}
			(Value::Item(Item::Unit), component)
		}
		Expression::TerminationLoop(_, _, _) => unimplemented!(),
		Expression::Mutation(mutation, mutable, expression) => {
			let (value, component) = basic(function, context, expression);
			let (variable, other) = basic(function, context, mutable);
			let component = context.join(component, other, span);

			match variable {
				Value::Location(location) => {
					let statement = Statement::Mutation(mutation.node.clone(), location, value);
					(Value::Item(Item::Unit), context.push(component, Spanned::new(statement, span)))
				}
				_ => panic!("Value expression cannot be mutated")
			}
		}
		Expression::ExplicitDrop(_, _) => unimplemented!(),
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
		// TODO: Resolve integers based on inference type
		Expression::Unsigned(integer) => (Value::Item(Item::Unsigned64(*integer)), context.component()),
		Expression::Signed(integer) => (Value::Item(Item::Signed64(*integer)), context.component()),
		Expression::Truth(truth) => (Value::Item(Item::Truth(*truth)), context.component()),
	}
}
