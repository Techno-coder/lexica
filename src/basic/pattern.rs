use std::sync::Arc;

use crate::inference::TypeContext;
use crate::node::{BindingPattern, BindingVariable, ExpressionPattern, FunctionContext,
	MutationKind, Pattern, VariablePattern};
use crate::span::{Span, Spanned};

use super::{BasicContext, Component, Compound, Instance, Item, Location, Projection,
	Statement, Value};

pub fn binding(context: &mut BasicContext, mut component: Component,
               pattern: &BindingPattern, location: Location) -> Component {
	match pattern {
		Pattern::Wildcard => panic!("Wildcard binding is invalid"),
		Pattern::Terminal(terminal) => {
			let BindingVariable(variable, _) = terminal.node.clone();
			let compound = Compound::Value(Value::Location(location));
			let statement = Statement::Binding(variable, compound);
			context.push(component, Spanned::new(statement, terminal.span))
		}
		Pattern::Tuple(patterns) => {
			for (index, pattern) in patterns.iter().enumerate() {
				let field: Arc<str> = index.to_string().into();
				let location = location.clone().push(Projection::Field(field));
				component = binding(context, component, pattern, location);
			}
			component
		}
	}
}

pub fn explicit_drop(context: &mut BasicContext, mut component: Component,
                     pattern: &VariablePattern, location: Location) -> Component {
	match pattern {
		Pattern::Wildcard => panic!("Wildcard explicit drop is invalid"),
		Pattern::Terminal(terminal) => {
			let compound = Compound::Value(Value::Location(location));
			let statement = Statement::Binding(terminal.node.clone(), compound);
			context.push(component, Spanned::new(statement, terminal.span))
		}
		Pattern::Tuple(patterns) => {
			for (index, pattern) in patterns.iter().enumerate() {
				let field: Arc<str> = index.to_string().into();
				let location = location.clone().push(Projection::Field(field));
				component = explicit_drop(context, component, pattern, location);
			}
			component
		}
	}
}

pub fn pattern(function: &FunctionContext, type_context: &TypeContext, context: &mut BasicContext,
               expression: &ExpressionPattern, span: Span) -> (Value, Component) {
	match expression {
		Pattern::Wildcard => panic!("Wildcard expression is not a value"),
		Pattern::Terminal(expression) => super::expression::basic(function,
			type_context, context, expression),
		Pattern::Tuple(patterns) => fields(context, Instance::tuple(), patterns.iter()
			.enumerate().map(|(index, expression)| (index.to_string().into(), expression)), span,
			&mut |context, expression| pattern(function, type_context, context, expression, span)),
	}
}

pub fn fields<T, F>(context: &mut BasicContext, mut instance: Instance,
                    fields: impl Iterator<Item=(Arc<str>, T)>, span: Span, function: &mut F)
                    -> (Value, Component) where F: FnMut(&mut BasicContext, T) -> (Value, Component) {
	let variable = context.temporary();
	let mut component = context.component();
	let mut statements = Vec::new();
	for (field, value) in fields {
		let (value, other) = function(context, value);
		instance.fields.insert(field.clone(), Item::Uninitialised);
		component = context.join(component, other, span);

		let projection = Projection::Field(field);
		let location = Location::new(variable.clone()).push(projection);
		let statement = Statement::Mutation(MutationKind::Assign, location, value);
		statements.push(Spanned::new(statement, span));
	}

	let compound = Compound::Value(Value::Item(Item::Instance(instance)));
	let statement = Statement::Binding(variable.clone(), compound);
	component = context.push(component, Spanned::new(statement, span));

	let component = statements.into_iter().fold(component,
		|component, statement| context.push(component, statement));
	(Value::Location(Location::new(variable)), component)
}
