use std::collections::HashMap;
use std::marker::PhantomData;

use crate::node::Identifier;

use super::NodeConstruct;

#[derive(Debug, Default)]
pub struct Context<'a> {
	bindings: HashMap<Identifier<'a>, i64>,
	evaluations: HashMap<EvaluationKey<'a>, i64>,
}

impl<'a> Context<'a> {
	pub fn register_binding(&mut self, identifier: Identifier<'a>, value: i64) {
		self.bindings.insert(identifier, value);
	}

	pub fn invalidate_binding(&mut self, identifier: &Identifier<'a>) {
		self.bindings.remove(identifier);
	}

	pub fn binding_value(&self, identifier: &Identifier<'a>) -> &i64 {
		self.bindings.get(identifier)
		    .expect(&format!("Binding does not exist for {:?}", identifier))
	}

	pub fn cache_evaluation(&mut self, node: &'a NodeConstruct, value: i64) {
		self.evaluations.insert(EvaluationKey::resolve(node), value);
	}

	pub fn invalidate_evaluation(&mut self, node: &'a NodeConstruct) {
		self.evaluations.remove(&EvaluationKey::resolve(node));
	}

	pub fn has_evaluated(&self, node: &'a NodeConstruct) -> bool {
		self.evaluations.contains_key(&EvaluationKey::resolve(node))
	}

	pub fn evaluation(&self, node: &'a NodeConstruct) -> &i64 {
		self.evaluations.get(&EvaluationKey::resolve(node))
		    .expect(&format!("Evaluation does not exist for {:?}", node))
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct EvaluationKey<'a> {
	pointer: *const usize,
	lifetime: PhantomData<&'a NodeConstruct<'a>>,
}

impl<'a> EvaluationKey<'a> {
	pub fn resolve(node: &'a NodeConstruct) -> Self {
		EvaluationKey {
			pointer: node as *const NodeConstruct as *const _,
			lifetime: PhantomData,
		}
	}
}
