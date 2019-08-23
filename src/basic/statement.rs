use std::fmt;

use crate::node::{Variable, VariableTarget};
use crate::source::Spanned;

use super::{Expression, FunctionCall, Value};

#[derive(Debug, Clone)]
pub enum Statement<'a> {
	Binding(Spanned<Binding<'a>>),
	Mutation(Spanned<Mutation<'a>>),
	Assignment(Spanned<Assignment<'a>>),
	FunctionCall(Spanned<FunctionCall<'a>>),
	ImplicitDrop(Spanned<ImplicitDrop<'a>>),
}

impl<'a> fmt::Display for Statement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Statement::Binding(binding) => write!(f, "{}", binding),
			Statement::Mutation(mutation) => write!(f, "{}", mutation),
			Statement::Assignment(assignment) => write!(f, "{}", assignment),
			Statement::FunctionCall(function_call) => write!(f, "{}", function_call),
			Statement::ImplicitDrop(implicit_drop) => write!(f, "{}", implicit_drop),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Binding<'a> {
	pub variable: Spanned<Variable<'a>>,
	pub value: Value<'a>,
}

impl<'a> fmt::Display for Binding<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "let {} = {}", self.variable, self.value)
	}
}

#[derive(Debug, Clone)]
pub enum Mutation<'a> {
	Swap(Spanned<VariableTarget<'a>>, Spanned<VariableTarget<'a>>),
	AddAssign(Spanned<VariableTarget<'a>>, Spanned<Expression<'a>>),
	MinusAssign(Spanned<VariableTarget<'a>>, Spanned<Expression<'a>>),
	MultiplyAssign(Spanned<VariableTarget<'a>>, Spanned<Expression<'a>>),
}

impl<'a> fmt::Display for Mutation<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Mutation::Swap(left, right) => write!(f, "{} <=> {}", left, right),
			Mutation::AddAssign(identifier, expression) => write!(f, "{} += {}", identifier, expression),
			Mutation::MinusAssign(identifier, expression) => write!(f, "{} -= {}", identifier, expression),
			Mutation::MultiplyAssign(identifier, expression) => write!(f, "{} *= {}", identifier, expression),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Assignment<'a> {
	pub target: Spanned<VariableTarget<'a>>,
	pub expression: Spanned<Expression<'a>>,
}

impl<'a> fmt::Display for Assignment<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} = {}", self.target, self.expression)
	}
}

#[derive(Debug, Clone)]
pub struct ImplicitDrop<'a> {
	pub target: Spanned<VariableTarget<'a>>,
}

impl<'a> fmt::Display for ImplicitDrop<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "drop {}", self.target)
	}
}
