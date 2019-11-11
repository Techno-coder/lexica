use std::ops::{Index, IndexMut};
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::span::{Span, Spanned};

use super::{AscriptionPattern, BindingPattern, Expression, ExpressionKey};

pub type FunctionTypes = CHashMap<Arc<FunctionPath>, Arc<FunctionType>>;
pub type NodeFunctions = CHashMap<Arc<FunctionPath>, Arc<NodeFunction>>;

#[derive(Debug, Clone)]
pub struct FunctionType {
	pub parameters: Vec<Spanned<Parameter>>,
	pub return_type: Spanned<AscriptionPattern>,
	pub function_byte_offset: usize,
}

impl FunctionType {
	pub fn new(parameters: Vec<Spanned<Parameter>>, return_type: Spanned<AscriptionPattern>,
	           function_byte_offset: usize) -> Self {
		FunctionType { parameters, return_type, function_byte_offset }
	}
}

#[derive(Debug, Clone)]
pub struct Parameter(pub BindingPattern, pub AscriptionPattern);

#[derive(Debug, Clone)]
pub struct NodeFunction {
	pub context: FunctionContext,
	pub expression: ExpressionKey,
	pub function_type: Arc<FunctionType>,
}

impl NodeFunction {
	pub fn new(context: FunctionContext, expression: ExpressionKey,
	           function_type: Arc<FunctionType>) -> NodeFunction {
		NodeFunction { context, expression, function_type }
	}
}

#[derive(Debug, Clone)]
pub struct FunctionContext {
	pub function_path: Arc<FunctionPath>,
	pub expressions: Vec<Spanned<Expression>>,
}

impl FunctionContext {
	pub fn new(function_path: Arc<FunctionPath>) -> Self {
		FunctionContext { function_path, expressions: Vec::new() }
	}
}

impl FunctionContext {
	pub fn register(&mut self, expression: Spanned<Expression>) -> ExpressionKey {
		let expression_key = ExpressionKey(self.expressions.len());
		self.expressions.push(expression);
		expression_key
	}

	/// Temporarily removes an expression and mutates it.
	pub fn apply<F, R>(&mut self, expression_key: &ExpressionKey, function: F) -> R
		where F: FnOnce(&mut Self, &mut Spanned<Expression>) -> R {
		let replacement = Spanned::new(Expression::Block(Vec::new()), Span::INTERNAL);
		let mut expression = std::mem::replace(&mut self[expression_key], replacement);
		let value = function(self, &mut expression);
		self[expression_key] = expression;
		value
	}

	/// Applies a function to all expressions in order.
	/// The traversal deepens if the function returns false.
	pub fn traverse<F, E>(&mut self, expression: &ExpressionKey, function: &mut F) -> Result<(), E>
		where F: FnMut(&mut Self, &mut Spanned<Expression>) -> Result<bool, E> {
		self.apply(expression, |context, expression| {
			if !function(context, expression)? {
				match &mut expression.node {
					Expression::Block(block) => block.iter().try_for_each(|expression|
						context.traverse(expression, function)),
					Expression::Binding(_, _, expression) =>
						context.traverse(expression, function),
					Expression::TerminationLoop(condition_start, condition_end, expression) => {
						context.traverse(condition_end, function)?;
						condition_start.as_mut().map(|condition_start|
							context.traverse(condition_start, function)).transpose()?;
						context.traverse(expression, function)
					}
					Expression::Conditional(branches) => branches.iter_mut()
						.try_for_each(|(condition_start, condition_end, expression)| {
							context.traverse(condition_start, function)?;
							condition_end.as_mut().map(|condition_end|
								context.traverse(condition_end, function)).transpose()?;
							context.traverse(expression, function)
						}),
					Expression::Mutation(_, mutable, expression) => {
						context.traverse(mutable, function)?;
						context.traverse(expression, function)
					}
					Expression::ExplicitDrop(_, expression) =>
						context.traverse(expression, function),
					Expression::Field(expression, _) =>
						context.traverse(expression, function),
					Expression::FunctionCall(_, expressions, _) => expressions.iter()
						.try_for_each(|expression| context.traverse(expression, function)),
					Expression::Unary(_, expression) =>
						context.traverse(expression, function),
					Expression::Binary(_, left, right) => {
						context.traverse(left, function)?;
						context.traverse(right, function)
					}
					Expression::Structure(_, expressions) => expressions.values()
						.try_for_each(|(_, expression)| context.traverse(expression, function)),
					Expression::Pattern(pattern) => pattern.apply(&mut |terminal|
						context.traverse(terminal, function)),
					Expression::Variable(_) | Expression::Integer(_) |
					Expression::Truth(_) | Expression::Item(_) => Ok(()),
				}?;
			}
			Ok(())
		})
	}
}

impl Index<&ExpressionKey> for FunctionContext {
	type Output = Spanned<Expression>;

	fn index(&self, index: &ExpressionKey) -> &Self::Output {
		let &ExpressionKey(index) = index;
		self.expressions.get(index).unwrap_or_else(||
			panic!("Expression key: {}, is not present in function context: {}", index,
				self.function_path))
	}
}

impl IndexMut<&ExpressionKey> for FunctionContext {
	fn index_mut(&mut self, index: &ExpressionKey) -> &mut Self::Output {
		let &ExpressionKey(index) = index;
		match self.expressions.get_mut(index) {
			Some(expression) => expression,
			None => panic!("Expression key: {}, is not present in function context: {}",
				index, self.function_path),
		}
	}
}

