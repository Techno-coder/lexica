use crate::node::{MutationKind, Variable};
use crate::span::Spanned;

use super::{Compound, Object, Value};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeTarget(pub usize);

#[derive(Debug)]
pub struct BasicNode {
	pub statements: Vec<Spanned<Statement>>,
	pub reverse: Spanned<Branch>,
	pub advance: Spanned<Branch>,
}

#[derive(Debug)]
pub enum Statement {
	Binding(Variable, Compound),
	Mutation(MutationKind, Variable, Value),
}

#[derive(Debug)]
pub enum Branch {
	Jump(NodeTarget),
	Divergence(Divergence),
	Return(Value),
	Unreachable,
}

#[derive(Debug)]
pub struct Divergence {
	pub discriminant: Value,
	pub branches: Vec<(Discriminant, NodeTarget)>,
	pub default: NodeTarget,
}

#[derive(Debug, PartialEq)]
pub struct Discriminant(pub u64);

impl From<&Object> for Discriminant {
	fn from(object: &Object) -> Self {
		Discriminant(match object {
			Object::Truth(truth) => match truth {
				false => 0,
				true => !0,
			},
			Object::Unsigned64(value) => *value,
			Object::Instance(_) | Object::Uninitialised =>
				panic!("Invalid discriminant object"),
		})
	}
}
