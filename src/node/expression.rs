use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::basic::Item;
use crate::declaration::{FunctionPath, StructurePath};
use crate::span::{Span, Spanned};

use super::{AscriptionPattern, BindingPattern, ExpressionPattern, Mutability,
	Variable, VariablePattern};

pub type ConditionStart = ExpressionKey;
pub type ConditionEnd = ExpressionKey;
pub type Branch = (ConditionStart, Option<ConditionEnd>, ExpressionKey);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct ExpressionKey(pub usize);

impl fmt::Debug for ExpressionKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let ExpressionKey(index) = self;
		write!(f, "ExpressionKey({})", index)
	}
}

#[derive(Debug, Clone)]
pub enum Expression {
	Block(Vec<ExpressionKey>),
	Binding(Spanned<BindingPattern>, Option<AscriptionPattern>, ExpressionKey),
	TerminationLoop(Option<ConditionStart>, ConditionEnd, ExpressionKey),
	Mutation(Spanned<MutationKind>, ExpressionKey, ExpressionKey),
	ExplicitDrop(VariablePattern, ExpressionKey),
	Unary(Spanned<UnaryOperator>, ExpressionKey),
	Binary(Spanned<BinaryOperator>, ExpressionKey, ExpressionKey),
	Conditional(Vec<Branch>),
	Field(ExpressionKey, Spanned<Arc<str>>),
//	MethodCall(ExpressionKey, Spanned<Arc<str>>, Vec<ExpressionKey>),
	FunctionCall(Spanned<FunctionPath>, Vec<ExpressionKey>, Execution),
	Structure(Spanned<StructurePath>, HashMap<Arc<str>, (Span, ExpressionKey)>),
	Pattern(ExpressionPattern),
	Variable(Variable),
	Integer(i128),
	Truth(bool),
	Item(Item),
}

#[derive(Debug, Clone)]
pub enum Execution {
	Compile,
	Runtime,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
	Negate,
//	Dereference,
	Reference(Mutability),
}

impl fmt::Display for UnaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			UnaryOperator::Negate => write!(f, "-"),
//			UnaryOperator::Dereference => write!(f, "*"),
			UnaryOperator::Reference(Mutability::Immutable) => write!(f, "&"),
			UnaryOperator::Reference(Mutability::Mutable) => write!(f, "~&"),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
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
	GreaterThan,
	GreaterEqual,
	LessThan,
	LessEqual,
	Equality,
}

impl fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			BinaryOperator::Arithmetic(arithmetic) => write!(f, "{}", arithmetic),
			BinaryOperator::GreaterThan => write!(f, ">"),
			BinaryOperator::GreaterEqual => write!(f, ">="),
			BinaryOperator::LessThan => write!(f, "<"),
			BinaryOperator::LessEqual => write!(f, "<="),
			BinaryOperator::Equality => write!(f, "=="),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum MutationKind {
	Arithmetic(Arithmetic),
	Assign,
	Swap,
}

impl fmt::Display for MutationKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MutationKind::Arithmetic(arithmetic) => write!(f, "{}=", arithmetic),
			MutationKind::Assign => write!(f, "="),
			MutationKind::Swap => write!(f, "<=>"),
		}
	}
}
