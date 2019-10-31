use std::ops::Index;

use crate::node::Variable;
use crate::span::Spanned;

use super::{BasicNode, NodeTarget};

#[derive(Debug)]
pub struct BasicFunction {
	pub parameters: Vec<Spanned<Variable>>,
	pub nodes: Vec<BasicNode>,
	pub entry: NodeTarget,
	pub exit: NodeTarget,
}

impl Index<&NodeTarget> for BasicFunction {
	type Output = BasicNode;

	fn index(&self, index: &NodeTarget) -> &Self::Output {
		let NodeTarget(index) = index;
		&self.nodes[*index]
	}
}
