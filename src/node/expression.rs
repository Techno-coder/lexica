use std::ops::Index;

use crate::declaration::FunctionPath;
use crate::span::Spanned;

use super::{Ascription, BindingPattern, FunctionContext, Variable};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ExpressionKey(usize);

impl FunctionContext {
	pub fn register(&mut self, expression: Spanned<Expression>) -> ExpressionKey {
		let expression_key = ExpressionKey(self.expressions.len());
		self.expressions.push(expression);
		expression_key
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

#[derive(Debug, Clone)]
pub enum Expression {
	Block(Vec<ExpressionKey>),
	Binding(BindingPattern, Option<Spanned<Ascription>>, ExpressionKey),
	TerminationLoop(Option<ExpressionKey>, ExpressionKey, ExpressionKey),
	Mutation(Spanned<MutationKind>, ExpressionKey, ExpressionKey),
	ExplicitDrop(Spanned<Variable>, ExpressionKey),

	Binary(Spanned<BinaryOperator>, ExpressionKey, ExpressionKey),
	//	Conditional(Vec<(ExpressionKey, Option<ExpressionKey>, ExpressionKey)>),
//	Accessor(ExpressionKey, Arc<str>),
//	AccessorCall(ExpressionKey, Arc<str>, Vec<ExpressionKey>),
	FunctionCall(Spanned<FunctionPath>, Vec<ExpressionKey>),
//	Tuple(Vec<ExpressionKey>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
	Add,
	Minus,
}

#[derive(Debug, Clone)]
pub enum MutationKind {
	Arithmetic(BinaryOperator),
	//	Assignment,
	Swap,
}
