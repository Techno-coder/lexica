use std::collections::HashMap;

use super::{Item, NodeTarget, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
	Jump(NodeTarget),
	Divergence(Divergence),
	Return(Value),
	Unreachable,
}

impl Branch {
	pub fn retarget(&mut self, targets: &HashMap<NodeTarget, NodeTarget>) {
		match self {
			Branch::Jump(current) => targets.get(current).iter()
				.for_each(|target| *current = **target),
			Branch::Divergence(divergence) => divergence.retarget(targets),
			_ => (),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Divergence {
	pub discriminant: Value,
	pub branches: Vec<(Discriminant, NodeTarget)>,
	pub default: NodeTarget,
}

impl Divergence {
	pub fn retarget(&mut self, targets: &HashMap<NodeTarget, NodeTarget>) {
		self.branches.iter_mut().filter_map(|(_, current)|
			targets.get(current).map(|target| (current, target)))
			.for_each(|(current, target)| *current = *target);
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Discriminant(pub u64);

impl From<&Item> for Discriminant {
	fn from(object: &Item) -> Self {
		Discriminant(match object {
			Item::Truth(truth) => match truth {
				false => 0,
				true => !0,
			},
			Item::Unsigned64(value) => *value,
			_ => panic!("Invalid discriminant object"),
		})
	}
}
