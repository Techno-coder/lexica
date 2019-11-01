use std::ops::{Index, Not};
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::FunctionPath;
use crate::node::Variable;
use crate::span::Spanned;

use super::{BasicNode, NodeTarget};

pub type BasicFunctions = CHashMap<Arc<FunctionPath>, Arc<BasicFunction>>;

#[derive(Debug)]
pub struct BasicFunction {
	pub parameters: Vec<Spanned<Variable>>,
	pub component: Component,
	pub nodes: Vec<BasicNode>,
}

impl Index<&NodeTarget> for BasicFunction {
	type Output = BasicNode;

	fn index(&self, index: &NodeTarget) -> &Self::Output {
		let NodeTarget(index) = index;
		&self.nodes[*index]
	}
}

#[derive(Debug)]
pub struct Component {
	pub entry: NodeTarget,
	pub exit: NodeTarget,
}

#[derive(Debug, Copy, Clone)]
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
