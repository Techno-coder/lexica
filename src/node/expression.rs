use std::fmt;
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
//	Field(ExpressionKey, Arc<str>),
//	MethodCall(ExpressionKey, Arc<str>, Vec<ExpressionKey>),
//	FunctionCall(Spanned<FunctionPath>, Vec<ExpressionKey>),

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

impl fmt::Display for Arithmetic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Arithmetic::Add => write!(f, "+"),
			Arithmetic::Minus => write!(f, "-"),
			Arithmetic::Multiply => write!(f, "*"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
	Arithmetic(Arithmetic),
	Equality,
}

impl fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			BinaryOperator::Arithmetic(arithmetic) => write!(f, "{}", arithmetic),
			BinaryOperator::Equality => write!(f, "=="),
		}
	}
}

#[derive(Debug, Clone)]
pub enum MutationKind {
	Arithmetic(Arithmetic),
	Assign,
	Swap,
}

impl fmt::Display for MutationKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MutationKind::Arithmetic(arithmetic) => write!(f, "{}", arithmetic),
			MutationKind::Assign => write!(f, "="),
			MutationKind::Swap => write!(f, "<=>"),
		}
	}
}
