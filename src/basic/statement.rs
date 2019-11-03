use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

use crate::extension::Indent;
use crate::node::{BinaryOperator, MutationKind, Variable};

#[derive(Debug)]
pub enum Statement {
	Binding(Variable, Compound),
	Mutation(MutationKind, Location, Value),
}

impl fmt::Display for Statement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Statement::Binding(variable, compound) =>
				write!(f, "let {} = {}", variable, compound),
			Statement::Mutation(mutation, location, value) =>
				write!(f, "{} {} {}", location, mutation, value),
		}
	}
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

impl fmt::Display for Compound {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Compound::Value(value) =>
				write!(f, "{}", value),
			Compound::Binary(operator, left, right) =>
				write!(f, "{} {} {}", left, operator, right),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Location(Location),
	Item(Item),
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Value::Location(location) => write!(f, "{}", location),
			Value::Item(item) => write!(f, "{}", item),
		}
	}
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

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Truth(truth) => write!(f, "{}", truth),
			Item::Signed64(integer) => write!(f, "{}", integer),
			Item::Unsigned64(integer) => write!(f, "{}", integer),
			Item::Instance(instance) => write!(f, "{}", instance),
			Item::Uninitialised => write!(f, "<!>"),
			Item::Unit => write!(f, "()"),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Instance {
	pub fields: HashMap<Arc<str>, Item>,
}

impl fmt::Display for Instance {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f)?;
		let indent = &mut Indent::new(f);
		for (index, (field, item)) in self.fields.iter().enumerate() {
			match item {
				Item::Instance(_) => write!(indent, "{}: {}", field, item),
				_ => write!(indent, "{}: {},", field, item),
			}?;

			if index < self.fields.len() - 1 { writeln!(indent)?; }
		}
		Ok(())
	}
}
