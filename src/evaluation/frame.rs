use std::collections::HashMap;
use std::sync::Arc;

use crate::basic::{BasicFunction, BasicNode, Item, Location, NodeTarget, Projection,
	Statement, Value};
use crate::node::Variable;
use crate::span::Spanned;

#[derive(Debug)]
pub struct EvaluationFrame {
	pub function: Arc<BasicFunction>,
	pub context: FrameContext,
}

impl EvaluationFrame {
	pub fn new(function: Arc<BasicFunction>) -> Self {
		let context = FrameContext::new(function.component.entry);
		EvaluationFrame { function, context }
	}
}

#[derive(Debug)]
pub struct FrameContext {
	variables: HashMap<Variable, Item>,
	pub current_node: NodeTarget,
	pub next_statement: usize,
}

impl FrameContext {
	pub fn new(current_node: NodeTarget) -> FrameContext {
		let variables = HashMap::new();
		FrameContext { variables, current_node, next_statement: 0 }
	}

	pub fn node<'a>(&self, function: &'a Arc<BasicFunction>) -> &'a BasicNode {
		&function[&self.current_node]
	}

	pub fn statement<'a>(&self, function: &'a Arc<BasicFunction>) -> &'a Spanned<Statement> {
		let node = &self.node(function);
		node.statements.get(self.next_statement).unwrap_or_else(||
			panic!("Statement index: {}, is invalid in node: {}, of length: {}",
				self.next_statement, self.current_node, node.statements.len()))
	}

	pub fn insert(&mut self, variable: Variable, object: Item) {
		if self.variables.insert(variable.clone(), object).is_some() {
			panic!("Variable: {}, is already bound in frame", variable);
		}
	}

	pub fn value<'a>(&'a self, value: &'a Value) -> &'a Item {
		match value {
			Value::Item(object) => object,
			Value::Location(location) => {
				let variable = self.variables.get(&location.variable).unwrap_or_else(||
					panic!("Variable: {}, does not exist in frame", location.variable));
				location.projections.iter().fold(variable, |variable, projection| {
					match projection {
						Projection::Field(field) => match variable {
							Item::Instance(instance) => instance.fields.get(field).unwrap(),
							_ => panic!("Field projection: {}, on item that is not instance", field)
						}
					}
				})
			}
		}
	}

	pub fn location<F, R>(&mut self, location: &Location, function: F) -> R
		where F: FnOnce(&mut Self, &mut Item) -> R {
		self.variable(&location.variable, |frame, variable| {
			function(frame, location.projections.iter().fold(variable, |variable, projection| {
				match projection {
					Projection::Field(field) => match variable {
						Item::Instance(instance) => instance.fields.get_mut(field).unwrap(),
						_ => panic!("Field projection: {}, on item that is not instance", field)
					}
				}
			}))
		})
	}

	fn variable<F, R>(&mut self, variable: &Variable, function: F) -> R
		where F: FnOnce(&mut Self, &mut Item) -> R {
		let (key, mut object) = self.variables.remove_entry(variable).unwrap();
		let value = function(self, &mut object);
		self.variables.insert(key, object);
		value
	}
}
