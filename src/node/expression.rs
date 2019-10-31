use crate::span::Spanned;

use super::{AscriptionPattern, BindingPattern, ExpressionPattern, Variable, VariablePattern};

pub type ConditionStart = ExpressionKey;
pub type ConditionEnd = ExpressionKey;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ExpressionKey(pub usize);

#[derive(Debug, Clone)]
pub enum Expression {
	Block(Vec<ExpressionKey>),
	Binding(Spanned<BindingPattern>, Option<AscriptionPattern>, ExpressionKey),
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
	Assign,
	Swap,
}
