use std::sync::Arc;

use crate::basic::{Instance, Item};
use crate::context::Context;
use crate::declaration::{ModulePath, StructurePath};
use crate::error::Diagnostic;
use crate::intrinsic::Intrinsic;
use crate::node::{Ascription, Pattern};
use crate::span::Spanned;

use super::*;

pub fn compile_root(context: &Context, function: &mut Function) -> Result<(), Diagnostic> {
	for index in 0..function.context.expressions.len() {
		let expression_key = &ExpressionKey(index);
		let expression = &function.context[expression_key].node;
		if let Expression::FunctionCall(_, _, Execution::Compile) = expression {
			let item = compile(context, &mut function.context, expression_key, None)?;
			function.context.apply(expression_key, |_, expression|
				expression.node = Expression::Item(item));
		}
	}
	Ok(())
}

fn compile(context: &Context, function: &mut FunctionContext, expression: &ExpressionKey,
           ascription: Option<&AscriptionPattern>) -> Result<Item, Diagnostic> {
	function.apply(expression, |function, expression| {
		let span = expression.span;
		match &mut expression.node {
			Expression::Item(item) => Ok(item.clone()),
			Expression::Truth(truth) => Ok(Item::Truth(*truth)),
			Expression::FunctionCall(function_path, expressions, Execution::Compile) => {
				let function_path = function_path.clone().map(Arc::new);
				let function_type = super::function_type(context, &function_path)?;

				let arguments = Iterator::zip(function_type.parameters.iter(), expressions.iter())
					.map(|(parameter, expression)| {
						let Parameter(_, ascription) = &parameter.node;
						compile(context, function, expression, Some(ascription))
					}).collect::<Result<_, _>>()?;

				crate::evaluation::evaluate(context, &function_path, arguments).map_err(|diagnostic|
					diagnostic.note(format!("Invoked from: {}", span.location(context))))
			}
			Expression::Integer(integer) => match ascription {
				Some(pattern) => {
					let error = Spanned::new(NodeError::ArgumentType(pattern.clone()), span);
					let error = Err(Diagnostic::new(error));
					match pattern {
						Pattern::Terminal(terminal) => {
							let Ascription(StructurePath(declaration_path)) = &terminal.node;
							let is_intrinsic = declaration_path.module_path == ModulePath::intrinsic();
							let intrinsic = Intrinsic::parse(&declaration_path.identifier)
								.and_then(|intrinsic| Item::integer(intrinsic, *integer));
							match (is_intrinsic, intrinsic) {
								(true, Some(item)) => Ok(item),
								_ => error,
							}
						}
						_ => error,
					}
				}
				None => Err(Diagnostic::new(Spanned::new(NodeError::RuntimeExpression, span))),
			},
			Expression::Pattern(expression) => pattern(context, function, expression, ascription),
			_ => Err(Diagnostic::new(Spanned::new(NodeError::RuntimeExpression, span))),
		}
	})
}

fn pattern(context: &Context, function: &mut FunctionContext, expression: &ExpressionPattern,
           ascription: Option<&AscriptionPattern>) -> Result<Item, Diagnostic> {
	match expression {
		Pattern::Wildcard => panic!("Wildcard expression is invalid"),
		Pattern::Terminal(terminal) => compile(context, function, terminal, ascription),
		Pattern::Tuple(patterns) => {
			let mut instance = Instance::tuple();
			for (index, expression) in patterns.iter().enumerate() {
				let field: Arc<str> = index.to_string().into();
				let ascription = ascription.and_then(|ascription| match ascription {
					Pattern::Tuple(patterns) => patterns.get(index),
					_ => None,
				});

				let item = pattern(context, function, expression, ascription)?;
				instance.fields.insert(field, item);
			}
			Ok(Item::Instance(instance))
		}
	}
}
