use std::sync::Arc;

use crate::node::ExpressionKey;
use crate::span::Spanned;

pub type VariablePattern = Pattern<Spanned<Variable>>;
pub type BindingPattern = Pattern<Spanned<BindingVariable>>;
pub type AscriptionPattern = Pattern<Spanned<Ascription>>;
pub type ExpressionPattern = Pattern<ExpressionKey>;

#[derive(Debug, Clone)]
pub struct Variable(pub Arc<str>, pub usize);

impl Variable {
	pub fn new(identifier: Arc<str>) -> Self {
		Variable(identifier, usize::max_value())
	}
}

#[derive(Debug, Clone)]
pub struct BindingVariable(pub Variable, pub Mutability);

#[derive(Debug, Copy, Clone)]
pub enum Mutability {
	Immutable,
	Mutable,
}

#[derive(Debug, Clone)]
pub struct Ascription(pub Arc<str>);

#[derive(Debug, Clone)]
pub enum Pattern<T> {
	Wildcard,
	Terminal(T),
	Tuple(Vec<Pattern<T>>),
}
