use std::ops::{Index, IndexMut};
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::span::{Span, Spanned};

use super::{AscriptionPattern, BindingPattern, Expression, ExpressionKey};

pub type FunctionTypes = CHashMap<Arc<FunctionPath>, Arc<FunctionType>>;
pub type NodeFunctions = CHashMap<Arc<FunctionPath>, Arc<Function>>;

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
pub struct Function {
	pub context: FunctionContext,
	pub expression: ExpressionKey,
	pub function_type: Arc<FunctionType>,
}

impl Function {
	pub fn new(context: FunctionContext, expression: ExpressionKey,
	           function_type: Arc<FunctionType>) -> Function {
		Function { context, expression, function_type }
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

	pub fn apply<F, R, E>(&mut self, expression_key: &ExpressionKey, function: &mut F) -> Result<R, E>
		where F: FnMut(&mut Self, &mut Spanned<Expression>) -> Result<R, E> {
		let replacement = Spanned::new(Expression::Block(Vec::new()), Span::INTERNAL);
		let mut expression = std::mem::replace(&mut self[expression_key], replacement);
		let result = function(self, &mut expression);
		self[expression_key] = expression;
		result
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

