use std::sync::Arc;

use crate::declaration::FunctionPath;
use crate::inference::{TypeContext, TypeResolution};
use crate::node::{Ascription, BindingVariable, Expression, ExpressionKey,
	FunctionContext, MutationKind, Parameter, Pattern, UnaryOperator};
use crate::span::Spanned;

use super::{BasicContext, Component, Compound, Direction, Instance, Item,
	Location, Projection, Statement, Value};

pub fn basic(function: &FunctionContext, type_context: &TypeContext,
             context: &mut BasicContext, expression_key: &ExpressionKey) -> (Value, Component) {
	let expression = &function[expression_key];
	let span = expression.span;
	match &expression.node {
		Expression::Block(block) => {
			context.push_frame();
			let (mut value, mut component) = (None, context.component());
			for expression in block {
				let (other_value, other) = basic(function, type_context, context, expression);
				component = context.join(component, other, span);
				value = Some(other_value);
			}

			let value = value.unwrap();
			context.consume_value(&value);
			let other = context.pop_frame();
			(value, context.join(component, other, span))
		}
		Expression::Binding(binding, _, expression) => {
			let (value, mut component) = basic(function, type_context, context, expression);
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
						component, &binding.node, location);
				}
			}
			(Value::Item(Item::Unit), component)
		}
		Expression::TerminationLoop(condition_start, condition_end, expression) =>
			super::conditional::termination(function, type_context, context,
				condition_start, condition_end, expression, span),
		Expression::Conditional(branches) =>
			super::conditional::conditional(function, type_context, context, branches, span),
		Expression::Mutation(mutation, mutable, expression) => {
			let (value, component) = basic(function, type_context, context, expression);
			let (variable, other) = basic(function, type_context, context, mutable);
			let mut component = context.join(component, other, span);
			match variable {
				Value::Location(location) => {
					if mutation.node == MutationKind::Assign && context.is_reversible() {
						let statement = Statement::ImplicitDrop(location.clone());
						component = context.push(component, Spanned::new(statement, span));
					}

					let statement = Statement::Mutation(mutation.node.clone(), location, value);
					(Value::Item(Item::Unit), context.push(component, Spanned::new(statement, span)))
				}
				_ => panic!("Value expression cannot be mutated")
			}
		}
		Expression::ExplicitDrop(_, _) if !context.is_reversible() =>
			(Value::Item(Item::Unit), context.component()),
		Expression::ExplicitDrop(variable, expression) => {
			variable.traverse(&mut |variable| context.consume_variable(&variable.node));
			let (value, component) = basic(function, type_context, context, expression);
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
		Expression::Field(expression, field) => {
			let (value, mut component) = basic(function, type_context, context, expression);
			let mut location = match value {
				Value::Location(location) => location,
				Value::Item(_) => {
					let variable = context.temporary();
					let statement = Statement::Binding(variable.clone(), Compound::Value(value));
					component = context.push(component, Spanned::new(statement, span));
					Location::new(variable)
				}
			};

			let mut resolution = &type_context[expression];
			while let TypeResolution::Reference(_, type_resolution) = resolution {
				location.projections.push(Projection::Dereference);
				resolution = type_resolution;
			}

			let location = location.push(Projection::Field(field.node.clone()));
			(Value::Location(location), component)
		}
		Expression::MethodCall(receiver, method, expressions) => {
			let mut values = Vec::new();
			let (mut value, mut component) = basic(function, type_context, context, receiver);

			let mut receiver = &type_context[receiver];
			let structure = loop {
				match receiver {
					TypeResolution::Instance(structure, _) => break structure,
					TypeResolution::Reference(_, resolution) => {
						match &mut value {
							Value::Location(location) => location
								.projections.push(Projection::Dereference),
							_ => panic!("Cannot dereference value that is not reference"),
						}
						receiver = resolution;
					}
					_ => panic!("Receiver type resolution must be instance or reference")
				}
			}.clone();

			let function_path = method.clone().map(|method|
				Arc::new(FunctionPath::method(structure, method)));
			let function_type = crate::node::function_type(context.context, &function_path).unwrap();
			let parameter = &function_type.parameters.first().unwrap().node;
			if let Parameter(_, Pattern::Terminal(ascription)) = parameter {
				if let Ascription::Reference(permission, _, _) = ascription.node {
					let variable = context.temporary();
					let compound = Compound::Unary(UnaryOperator::Reference(permission), value);
					let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
					value = Value::Location(Location::new(variable));
					component = context.push(component, statement);
				}
			}

			values.push(value);
			for expression in expressions {
				let (value, other) = basic(function, type_context, context, expression);
				component = context.join(component, other, function[expression].span);
				values.push(value);
			}

			let variable = context.temporary();
			let compound = Compound::FunctionCall(function_path, values);
			let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
			(Value::Location(Location::new(variable)), context.push(component, statement))
		}
		Expression::FunctionCall(function_path, expressions, _) => {
			let mut values = Vec::new();
			let mut component = context.component();
			for expression in expressions {
				let (value, other) = basic(function, type_context, context, expression);
				component = context.join(component, other, function[expression].span);
				values.push(value);
			}

			let variable = context.temporary();
			let compound = Compound::FunctionCall(function_path.clone()
				.map(|function_path| Arc::new(function_path)), values);
			let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
			(Value::Location(Location::new(variable)), context.push(component, statement))
		}
		Expression::Unary(operator, expression) => {
			let variable = context.temporary();
			let (mut value, mut component) = basic(function, type_context, context, expression);
			if let (UnaryOperator::Reference(_), Value::Item(_)) = (&operator.node, &value) {
				let variable = context.temporary();
				let statement = Statement::Binding(variable.clone(), Compound::Value(value));
				component = context.push(component, Spanned::new(statement, span));
				value = Value::Location(Location::new(variable));
			}

			match (&operator.node, &mut value) {
				(UnaryOperator::Dereference, Value::Location(location)) => {
					location.projections.push(Projection::Dereference);
					(value, component)
				}
				_ => {
					let compound = Compound::Unary(operator.node.clone(), value);
					let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
					(Value::Location(Location::new(variable)), context.push(component, statement))
				}
			}
		}
		Expression::Binary(operator, left, right) => {
			let (left_value, left) = basic(function, type_context, context, left);
			let (right_value, right) = basic(function, type_context, context, right);
			let component = context.join(left, right, span);

			let variable = context.temporary();
			let compound = Compound::Binary(operator.node.clone(), left_value, right_value);
			let statement = Spanned::new(Statement::Binding(variable.clone(), compound), span);
			(Value::Location(Location::new(variable)), context.push(component, statement))
		}
		Expression::Structure(_, expressions) => {
			let instance = Instance::new(type_context[expression_key].clone());
			super::pattern::fields(context, instance, expressions.iter()
				.map(|(field, (_, expression))| (field.clone(), expression)), span,
				&mut |context, expression| basic(function, type_context, context, expression))
		}
		Expression::Pattern(expression) =>
			super::pattern::pattern(function, type_context, context, expression, span),
		Expression::Variable(variable) =>
			(Value::Location(Location::new(variable.clone())), context.component()),
		Expression::Integer(integer) => {
			let resolution = &type_context[expression_key];
			(Value::Item(resolution.intrinsic()
				.and_then(|intrinsic| Item::integer(intrinsic, *integer))
				.unwrap_or_else(|| panic!("Type: {:?}, is not of intrinsic integer", resolution))),
				context.component())
		}
		Expression::Truth(truth) => (Value::Item(Item::Truth(*truth)), context.component()),
		Expression::Item(item) => (Value::Item(item.clone()), context.component()),
	}
}
