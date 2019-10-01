use std::sync::Arc;

use crate::span::Spanned;

pub type BindingPattern = Pattern<Spanned<BindingVariable>>;
pub type AscriptionPattern = Pattern<Spanned<Ascription>>;

#[derive(Debug, Clone)]
pub struct Variable(pub Arc<str>, pub usize);

impl Variable {
	pub fn new(identifier: Arc<str>) -> Self {
		Variable(identifier, usize::max_value())
	}

	pub fn is_internal(&self) -> bool {
		let Variable(_, generation) = self;
		generation == &usize::max_value()
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
