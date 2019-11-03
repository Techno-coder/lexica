use std::fmt;
use std::sync::Arc;

use crate::declaration::StructurePath;
use crate::node::ExpressionKey;
use crate::span::Spanned;

pub type VariablePattern = Pattern<Spanned<Variable>>;
pub type BindingPattern = Pattern<Spanned<BindingVariable>>;
pub type AscriptionPattern = Pattern<Spanned<Ascription>>;
pub type ExpressionPattern = Pattern<ExpressionKey>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Variable(pub Arc<str>, pub usize);

impl Variable {
	const TEMPORARY: &'static str = "'";

	pub fn new(identifier: Arc<str>) -> Self {
		Variable(identifier, usize::max_value())
	}

	pub fn new_parameter(identifier: Arc<str>) -> Self {
		Variable(identifier, 0)
	}

	pub fn new_temporary(generation: usize) -> Self {
		Variable(Self::TEMPORARY.into(), generation)
	}
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Variable(identifier, generation) = self;
		match identifier.as_ref() == Self::TEMPORARY {
			true => write!(f, "{}{}", generation, identifier),
			false => write!(f, "{}", identifier),
		}
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
pub struct Ascription(pub StructurePath);

#[derive(Debug, Clone)]
pub enum Pattern<T> {
	Wildcard,
	Terminal(T),
	Tuple(Vec<Pattern<T>>),
}

impl<T> Pattern<T> {
	pub fn apply<F, E>(&mut self, function: &mut F) -> Result<(), E>
		where F: FnMut(&mut T) -> Result<(), E> {
		match self {
			Pattern::Wildcard => Ok(()),
			Pattern::Terminal(terminal) => function(terminal),
			Pattern::Tuple(patterns) => patterns.iter_mut()
				.try_for_each(|pattern| pattern.apply(function)),
		}
	}

	pub fn traverse<F, E>(&self, function: &mut F) -> Result<(), E>
		where F: FnMut(&T) -> Result<(), E> {
		match self {
			Pattern::Wildcard => Ok(()),
			Pattern::Terminal(terminal) => function(terminal),
			Pattern::Tuple(patterns) => patterns.iter()
				.try_for_each(|pattern| pattern.traverse(function)),
		}
	}
}
