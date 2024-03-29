use std::fmt::{self, Write};
use std::ops::{BitXor, Index, Not};
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::extension::Indent;
use crate::inference::TypeResolution;

use super::{BasicNode, NodeTarget};

pub type BasicFunctions = CHashMap<(Arc<FunctionPath>, Reversibility), Arc<BasicFunction>>;

#[derive(Debug)]
pub struct BasicFunction {
	pub parameters: Vec<TypeResolution>,
	pub component: Component,
	pub nodes: Vec<BasicNode>,
}

impl BasicFunction {
	pub fn parameter_type(&self) -> TypeResolution {
		let path = crate::intrinsic::Intrinsic::Tuple.structure();
		TypeResolution::Instance(path, self.parameters.clone())
	}
}

impl Index<&NodeTarget> for BasicFunction {
	type Output = BasicNode;

	fn index(&self, index: &NodeTarget) -> &Self::Output {
		let NodeTarget(index) = index;
		&self.nodes[*index]
	}
}

impl fmt::Display for BasicFunction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "(")?;
		if let Some((last, slice)) = self.parameters.split_last() {
			slice.iter().try_for_each(|parameter| write!(f, "{}, ", parameter))?;
			write!(f, "{}", last)?;
		}

		writeln!(f, "):")?;
		let indent = &mut Indent::new(f);
		self.nodes.iter().enumerate().try_for_each(|(index, node)| {
			if NodeTarget(index) == self.component.entry { write!(indent, "-")?; }
			if NodeTarget(index) == self.component.exit { write!(indent, "+")?; }

			match node.direction {
				Direction::Advance => writeln!(indent, "{}:", index),
				Direction::Reverse => writeln!(indent, "!{}:", index),
			}?;

			let indent = &mut Indent::new(indent);
			writeln!(indent, "{}", node)
		})
	}
}

#[derive(Debug)]
pub struct Component {
	pub entry: NodeTarget,
	pub exit: NodeTarget,
}

impl Component {
	pub fn new(entry: NodeTarget, exit: NodeTarget) -> Self {
		Component { entry, exit }
	}

	pub fn endpoint(&self, direction: Direction) -> NodeTarget {
		match direction {
			Direction::Advance => self.exit,
			Direction::Reverse => self.entry,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
	Advance,
	Reverse,
}

impl Not for Direction {
	type Output = Direction;

	fn not(self) -> Self::Output {
		match self {
			Direction::Advance => Direction::Reverse,
			Direction::Reverse => Direction::Advance,
		}
	}
}

impl BitXor for Direction {
	type Output = Direction;

	fn bitxor(self, other: Self) -> Self::Output {
		match self == other {
			true => Direction::Advance,
			false => Direction::Reverse,
		}
	}
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Reversibility {
	Reversible,
	Entropic,
}

impl From<Direction> for Reversibility {
	fn from(direction: Direction) -> Self {
		match direction {
			Direction::Advance => Reversibility::Entropic,
			Direction::Reverse => Reversibility::Reversible,
		}
	}
}
