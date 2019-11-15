use std::collections::HashMap;
use std::fmt::{self, Write};

use crate::extension::Indent;

use super::{Item, NodeTarget, Value};

#[derive(Clone, PartialEq)]
pub enum Branch {
	Jump(NodeTarget),
	Divergence(Divergence),
	Return(Value),
	Unreachable,
}

impl Branch {
	pub fn targets<'a>(&'a self) -> Box<dyn Iterator<Item=&NodeTarget> + 'a> {
		match &self {
			Branch::Jump(target) => Box::new(std::iter::once(target)),
			Branch::Divergence(divergence) => Box::new(divergence.targets()),
			Branch::Return(_) | Branch::Unreachable => Box::new(std::iter::empty()),
		}
	}

	pub fn retarget(&mut self, targets: &HashMap<NodeTarget, NodeTarget>) {
		match self {
			Branch::Jump(current) => targets.get(current).iter()
				.for_each(|target| *current = **target),
			Branch::Divergence(divergence) => divergence.retarget(targets),
			_ => (),
		}
	}
}

impl fmt::Display for Branch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Branch::Jump(target) => write!(f, "jump {}", target),
			Branch::Divergence(divergence) => write!(f, "{}", divergence),
			Branch::Return(value) => write!(f, "return {}", value),
			Branch::Unreachable => write!(f, "<!>"),
		}
	}
}

impl fmt::Debug for Branch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Branch::Jump(target) => write!(f, "Jump({:?})", target),
			Branch::Divergence(divergence) => write!(f, "Divergence({:#?})", divergence),
			Branch::Return(value) => write!(f, "Return({:?})", value),
			Branch::Unreachable => write!(f, "Unreachable"),
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
	pub fn truth(discriminant: Value, target: NodeTarget, default: NodeTarget) -> Self {
		let branches = vec![(Discriminant::item(Item::Truth(true)), target)];
		Divergence { discriminant, branches, default }
	}

	pub fn targets(&self) -> impl Iterator<Item=&NodeTarget> {
		self.branches.iter().map(|(_, target)| target)
			.chain(std::iter::once(&self.default))
	}

	pub fn retarget(&mut self, targets: &HashMap<NodeTarget, NodeTarget>) {
		std::iter::once(&mut self.default)
			.chain(self.branches.iter_mut().map(|(_, current)| current))
			.filter_map(|current| targets.get(current).map(|target| (current, target)))
			.for_each(|(current, target)| *current = *target);
	}
}

impl fmt::Display for Divergence {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "diverge {}:", self.discriminant)?;
		let indent = &mut Indent::new(f);
		self.branches.iter().try_for_each(|(discriminant, target)|
			writeln!(indent, "{} -> {}", discriminant, target))?;
		write!(indent, "_ -> {}", self.default)
	}
}

#[derive(Clone, PartialEq)]
pub struct Discriminant(pub u64);

impl Discriminant {
	pub fn item(item: Item) -> Self {
		Discriminant(match item {
			Item::Truth(truth) => match truth {
				false => 0,
				true => !0,
			},
			Item::Unsigned64(value) => value,
			_ => panic!("Invalid discriminant item"),
		})
	}
}

impl fmt::Display for Discriminant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Discriminant(discriminant) = self;
		match discriminant == &!0 {
			false => write!(f, "{:#x}", discriminant),
			true => write!(f, "true"),
		}
	}
}

impl fmt::Debug for Discriminant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Discriminant({})", self)
	}
}
