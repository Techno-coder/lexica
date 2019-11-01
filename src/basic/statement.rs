use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::node::{BinaryOperator, MutationKind, Variable};

#[derive(Debug)]
pub enum Statement {
	Binding(Variable, Compound),
	Mutation(MutationKind, Location, Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
	pub variable: Variable,
	pub projections: Vec<Projection>,
}

impl Location {
	pub fn new(variable: Variable) -> Self {
		Location { variable, projections: Vec::new() }
	}

	pub fn push(mut self, projection: Projection) -> Self {
		self.projections.push(projection);
		self
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.variable)?;
		self.projections.iter().try_for_each(|projection| write!(f, "{}", projection))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Projection {
	Field(Arc<str>),
}

impl fmt::Display for Projection {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Projection::Field(field) => write!(f, ".{}", field),
		}
	}
}

#[derive(Debug)]
pub enum Compound {
	Value(Value),
	Binary(BinaryOperator, Value, Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Location(Location),
	Item(Item),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
	Truth(bool),
	Signed64(i64),
	Unsigned64(u64),
	Instance(Instance),
	Uninitialised,
	Unit,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instance {
	pub fields: HashMap<Arc<str>, Item>,
}
