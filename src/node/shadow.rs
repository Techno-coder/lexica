use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::error::Diagnostic;
use crate::node::{Expression, FunctionType};
use crate::span::{Span, Spanned};

use super::{BindingVariable, ExpressionKey, Function, FunctionContext, NodeError,
	Parameter, Variable};

#[derive(Debug, Default)]
struct ShadowFrame {
	generations: HashMap<Arc<str>, usize>,
	drops: HashSet<Arc<str>>,
}

#[derive(Debug, Default)]
struct ShadowContext {
	frames: Vec<ShadowFrame>,
}

impl ShadowContext {
	fn register_variable(&mut self, variable: &mut BindingVariable) {
		let BindingVariable(Variable(identifier, generation), _) = variable;
		*generation = self.frames.iter().rev()
			.find_map(|frame| frame.generations.get(identifier))
			.map(|generation| generation + 1).unwrap_or(0);

		let frame = self.frame();
		frame.generations.insert(identifier.clone(), *generation);
		frame.drops.remove(identifier);
	}

	fn resolve_variable(&self, variable: &mut Variable, span: Span) -> Result<(), Diagnostic> {
		let Variable(identifier, generation) = variable;
		for frame in self.frames.iter().rev() {
			if frame.drops.contains(identifier) {
				let error = NodeError::DroppedVariable(identifier.clone());
				return Err(Diagnostic::new(Spanned::new(error, span)));
			}

			if let Some(variable_generation) = frame.generations.get(identifier) {
				*generation = *variable_generation;
				return Ok(());
			}
		}

		let error = NodeError::UndefinedVariable(identifier.clone());
		Err(Diagnostic::new(Spanned::new(error, span)))
	}

	fn new_frame(&mut self) {
		self.frames.push(ShadowFrame::default());
	}

	fn pop_frame(&mut self) {
		assert!(self.frames.pop().is_some());
	}

	fn frame(&mut self) -> &mut ShadowFrame {
		self.frames.last_mut().expect("Shadow frame stack is empty")
	}
}

pub fn shadow_function_type(function_type: &mut FunctionType) -> Result<(), Diagnostic> {
	let mut variables = HashSet::new();
	for parameter in &mut function_type.parameters {
		let Parameter(pattern, _) = &mut parameter.node;
		pattern.apply(&mut |terminal| {
			let BindingVariable(variable, _) = &mut terminal.node;
			let Variable(identifier, _) = variable;
			match variables.contains(identifier) {
				true => {
					let error = NodeError::DuplicateParameter(identifier.clone());
					Err(Diagnostic::new(Spanned::new(error, terminal.span)))
				}
				false => {
					variables.insert(identifier.clone());
					*variable = Variable::new_parameter(identifier.clone());
					Ok(())
				}
			}
		})?;
	}
	Ok(())
}

pub fn shadow_function(function: &mut Function) -> Result<(), Diagnostic> {
	let context = &mut ShadowContext::default();
	context.new_frame();

	for parameter in &function.function_type.parameters {
		let Parameter(pattern, _) = &parameter.node;
		pattern.traverse(&mut |terminal| {
			let BindingVariable(Variable(identifier, _), _) = &terminal.node;
			assert!(!context.frame().generations.contains_key(identifier));
			Ok(context.register_variable(&mut terminal.node.clone()))
		})?;
	}

	shadow(&mut function.context, context, &function.expression)?;
	context.pop_frame();
	Ok(())
}

fn shadow(function: &mut FunctionContext, context: &mut ShadowContext,
          expression: &ExpressionKey) -> Result<(), Diagnostic> {
	function.apply(expression, |function, expression| {
		match &mut expression.node {
			Expression::Block(block) => {
				context.new_frame();
				block.iter().try_for_each(|expression|
					shadow(function, context, expression))?;
				context.pop_frame();
				Ok(())
			}
			Expression::Binding(pattern, _, expression) => {
				shadow(function, context, expression)?;
				pattern.node.apply(&mut |terminal|
					Ok(context.register_variable(&mut terminal.node)))
			}
			Expression::TerminationLoop(start_condition, end_condition, expression) => {
				shadow(function, context, end_condition)?;
				start_condition.as_mut().map(|start_condition|
					shadow(function, context, start_condition)).transpose()?;
				shadow(function, context, expression)
			}
			Expression::Mutation(_, mutable, expression) => {
				shadow(function, context, mutable)?;
				shadow(function, context, expression)
			}
			Expression::ExplicitDrop(pattern, expression) => {
				pattern.apply(&mut |terminal| {
					context.resolve_variable(&mut terminal.node, terminal.span)?;
					let Variable(identifier, _) = &terminal.node;
					context.frame().drops.insert(identifier.clone());
					Ok(())
				})?;
				shadow(function, context, expression)
			}
			Expression::Binary(_, left, right) => {
				shadow(function, context, left)?;
				shadow(function, context, right)
			}
			Expression::Pattern(pattern) => pattern.apply(&mut |terminal|
				shadow(function, context, terminal)),
			Expression::Variable(variable) => context.resolve_variable(variable, expression.span),
			Expression::Unsigned(_) | Expression::Signed(_) | Expression::Truth(_) => Ok(()),
		}
	})
}
