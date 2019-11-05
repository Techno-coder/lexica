use std::sync::Arc;

use crate::inference::TypeContext;
use crate::node::{BindingPattern, BindingVariable, ExpressionPattern, FunctionContext,
	MutationKind, Pattern, VariablePattern};
use crate::span::{Span, Spanned};

use super::{BasicContext, Component, Compound, Instance, Item, Location, Projection,
	Statement, Value};

pub fn binding(context: &mut BasicContext, mut component: Component, value: &Value,
               pattern: &BindingPattern, location: Location) -> Component {
	match &pattern {
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
				component = binding(context, component, value, pattern, location);
			}
			component
		}
	}
}

pub fn explicit_drop(context: &mut BasicContext, mut component: Component, value: &Value,
                     pattern: &VariablePattern, location: Location) -> Component {
	match &pattern {
		Pattern::Wildcard => panic!("Wildcard explicit drop is invalid"),
		Pattern::Terminal(terminal) => {
			let value = Value::Location(location);
			let location = Location::new(terminal.node.clone());
			let statement = Statement::Mutation(MutationKind::Assign, location, value);
			context.push(component, Spanned::new(statement, terminal.span))
		}
		Pattern::Tuple(patterns) => {
			for (index, pattern) in patterns.iter().enumerate() {
				let field: Arc<str> = index.to_string().into();
				let location = location.clone().push(Projection::Field(field));
				component = explicit_drop(context, component, value, pattern, location);
			}
			component
		}
	}
}

pub fn pattern(function: &FunctionContext, context: &mut BasicContext, type_context: &TypeContext,
               expression: &ExpressionPattern, span: Span) -> (Value, Component) {
	match expression {
		Pattern::Wildcard => panic!("Wildcard expression is not a value"),
		Pattern::Terminal(expression) => super::function::basic(function,
			context, type_context, expression),
		Pattern::Tuple(patterns) => {
			let variable = context.temporary();
			let mut instance = Instance::default();
			let mut component = context.component();

			let mut statements = Vec::new();
			for (index, expression) in patterns.iter().enumerate() {
				let field: Arc<str> = index.to_string().into();
				let (value, other) = pattern(function, context, type_context, expression, span);
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
	}
}
