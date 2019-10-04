use std::ops::Index;

use crate::span::Spanned;

use super::{AscriptionPattern, BindingPattern, ExpressionPattern, FunctionContext,
	Variable, VariablePattern};

pub type ConditionStart = ExpressionKey;
pub type ConditionEnd = ExpressionKey;

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
	Binding(BindingPattern, Option<AscriptionPattern>, ExpressionKey),
	TerminationLoop(Option<ConditionStart>, ConditionEnd, ExpressionKey),
	Mutation(Spanned<MutationKind>, ExpressionKey, ExpressionKey),
	ExplicitDrop(VariablePattern, ExpressionKey),

	Binary(Spanned<BinaryOperator>, ExpressionKey, ExpressionKey),
	//	Conditional(Vec<(ConditionStart, Option<ConditionEnd>, ExpressionKey)>),
//	Accessor(ExpressionKey, Arc<str>),
//	AccessorCall(ExpressionKey, Arc<str>, Vec<ExpressionKey>),
//	FunctionCall(Spanned<FunctionPath>, Vec<ExpressionKey>),
//	Tuple(Vec<ExpressionKey>),

	Pattern(ExpressionPattern),
	Variable(Variable),
	Unsigned(u64),
	Signed(i64),
	Truth(bool),
}

#[derive(Debug, Clone)]
pub enum Arithmetic {
	Add,
	Minus,
	Multiply,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
	Arithmetic(Arithmetic),
	Equality,
}

#[derive(Debug, Clone)]
pub enum MutationKind {
	Arithmetic(Arithmetic),
	//	Assignment,
	Swap,
}
