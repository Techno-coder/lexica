use std::collections::HashMap;
use std::sync::Arc;

use crate::node::{BinaryOperator, Variable};

#[derive(Debug)]
pub enum Compound {
	Value(Value),
	Binary(BinaryOperator, Value, Value),
}

#[derive(Debug)]
pub enum Value {
	Variable(Variable),
	Object(Object),
}

#[derive(Debug, Clone)]
pub enum Object {
	Truth(bool),
	Unsigned64(u64),
	Instance(Instance),
	Uninitialised,
}

#[derive(Debug, Clone)]
pub struct Instance {
	fields: HashMap<Arc<str>, Object>,
}
