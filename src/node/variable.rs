use std::fmt;
use std::sync::Arc;

use crate::declaration::StructurePath;
use crate::node::ExpressionKey;
use crate::span::Spanned;

pub type VariablePattern = Pattern<Spanned<Variable>>;
pub type BindingPattern = Pattern<Spanned<BindingVariable>>;
pub type AscriptionPattern = Pattern<Spanned<Ascription>>;
pub type ExpressionPattern = Pattern<ExpressionKey>;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Variable(pub Arc<str>, pub usize);

impl Variable {
	const TEMPORARY: &'static str = "'";
	const UNRESOLVED: usize = usize::max_value();

	pub fn new(identifier: Arc<str>) -> Self {
		Variable(identifier, Self::UNRESOLVED)
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

impl fmt::Debug for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Variable(identifier, generation) = self;
		match identifier.as_ref() == Self::TEMPORARY {
			true => write!(f, "Variable(TEMPORARY, "),
			false => write!(f, "Variable({}, ", identifier),
		}?;

		match generation {
			&Self::UNRESOLVED => write!(f, "<?>)"),
			_ => write!(f, "{})", generation)
		}
	}
}

#[derive(Clone)]
pub struct BindingVariable(pub Variable, pub Mutability);

impl fmt::Debug for BindingVariable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let BindingVariable(Variable(identifier, generation), mutability) = self;
		write!(f, "BindingVariable({}, {}, {:?})", identifier, generation, mutability)
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Mutability {
	Immutable,
	Mutable,
}

#[derive(Clone)]
pub struct Ascription(pub StructurePath);

impl fmt::Debug for Ascription {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Ascription(structure_path) = self;
		write!(f, "Ascription({})", structure_path)
	}
}

impl fmt::Display for Ascription {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Ascription(structure_path) = self;
		write!(f, "{}", structure_path)
	}
}

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

	pub fn traverse<F>(&self, function: &mut F) where F: FnMut(&T) {
		match self {
			Pattern::Wildcard => (),
			Pattern::Terminal(terminal) => function(terminal),
			Pattern::Tuple(patterns) => patterns.iter()
				.for_each(|pattern| pattern.traverse(function)),
		}
	}
}

impl<T> fmt::Display for Pattern<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Pattern::Wildcard => write!(f, "_"),
			Pattern::Terminal(terminal) => write!(f, "{}", terminal),
			Pattern::Tuple(patterns) => {
				write!(f, "(")?;
				patterns.split_last().map(|(last, rest)| {
					rest.iter().try_for_each(|pattern| write!(f, "{}, ", pattern))?;
					write!(f, "{}", last)
				}).transpose()?;
				write!(f, ")")
			}
		}
	}
}
