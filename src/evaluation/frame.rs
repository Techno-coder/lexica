use std::collections::HashMap;
use std::sync::Arc;

use crate::basic::{BasicFunction, BasicNode, NodeTarget, Object, Statement, Value};
use crate::node::Variable;
use crate::span::Spanned;

#[derive(Debug)]
pub struct EvaluationFrame {
	pub function: Arc<BasicFunction>,
	pub context: FrameContext,
}

impl EvaluationFrame {
	pub fn new(function: Arc<BasicFunction>) -> Self {
		let mut context = FrameContext::new(function.entry);
		function.parameters.iter().for_each(|parameter|
			context.insert(parameter.node.clone(), Object::Uninitialised));
		EvaluationFrame { function, context }
	}
}

#[derive(Debug)]
pub struct FrameContext {
	variables: HashMap<Variable, Object>,
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
		&self.node(function).statements[self.next_statement]
	}

	pub fn insert(&mut self, variable: Variable, object: Object) {
		self.variables.insert(variable, object);
	}

	pub fn value<'a>(&'a self, value: &'a Value) -> &'a Object {
		match value {
			Value::Object(object) => object,
			Value::Variable(variable) => self.variables.get(variable).unwrap(),
		}
	}

	pub fn variable<F, R>(&mut self, variable: &Variable, function: F) -> R
		where F: FnOnce(&mut Self, &mut Object) -> R {
		let (key, mut object) = self.variables.remove_entry(variable).unwrap();
		let value = function(self, &mut object);
		self.variables.insert(key, object);
		value
	}
}
