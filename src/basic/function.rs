use std::sync::Arc;

use crate::context::Context;
use crate::declaration::FunctionPath;
use crate::error::Diagnostic;
use crate::node::{BindingVariable, ExpressionKey, Parameter};
use crate::span::Spanned;

use super::{BasicContext, BasicFunction, Branch, Location, Reversibility};
use super::expression::basic;

/// Lowers a partially evaluated function.
pub fn function(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                reversibility: Reversibility) -> Result<Arc<BasicFunction>, Diagnostic> {
	let context_key = (function_path.node.clone(), reversibility);
	if let Some(function) = context.basic_functions.get(&context_key) {
		return Ok(function.clone());
	}

	let mut parameters = Vec::new();
	let function = crate::evaluation::partial_function(context, function_path)?;
	let span = function.context[&function.expression].span;
	function.function_type.parameters.iter().map(|parameter| &parameter.node)
		.for_each(|Parameter(pattern, _)| pattern.traverse(&mut |terminal|
			parameters.push(terminal.clone().map(|BindingVariable(variable, _)| variable))));

	let mut basic_context = BasicContext::new(reversibility);
	let mut component = function.function_type.parameters.iter().map(|parameter| &parameter.node)
		.fold(basic_context.component(), |component, Parameter(pattern, _)| {
			let location = Location::new(basic_context.temporary());
			super::pattern::binding(&mut basic_context, component, &pattern, location)
		});

	let type_context = crate::inference::function(context, function_path)?;
	let (value, other) = basic(&function.context, &type_context,
		&mut basic_context, &function.expression);
	component = basic_context.join(component, other, span);

	basic_context.consume_value(&value);
	let other = basic_context.pop_frame();
	component = basic_context.join(component, other, span);
	let return_branch = Spanned::new(Branch::Return(value), span);
	basic_context[&component.exit].advance = return_branch;

	let (nodes, component) = basic_context.flatten(component);
	let function = Arc::new(BasicFunction { parameters, component, nodes });
	context.basic_functions.insert(context_key, function.clone());
	Ok(function)
}

/// Lowers an expression in a function. The function is not partially evaluated.
pub fn expression(context: &Context, function_path: &Spanned<Arc<FunctionPath>>,
                  expression: &ExpressionKey, reversibility: Reversibility)
                  -> Result<BasicFunction, Diagnostic> {
	let function = crate::node::function(context, function_path)?;
	let type_context = crate::inference::function(context, function_path)?;

	let mut basic_context = BasicContext::new(reversibility);
	let (value, component) = basic(&function.context, &type_context,
		&mut basic_context, expression);

	let return_branch = Spanned::new(Branch::Return(value), function_path.span);
	basic_context[&component.exit].advance = return_branch;
	let (nodes, component) = basic_context.flatten(component);

	let parameters = Vec::new();
	Ok(BasicFunction { parameters, component, nodes })
}
