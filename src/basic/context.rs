use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::node::Variable;
use crate::span::{Span, Spanned};

use super::{BasicNode, Branch, Component, Direction, NodeTarget, Statement};

#[derive(Debug, Default)]
pub struct BasicContext {
	next_temporary: usize,
	nodes: HashMap<NodeTarget, BasicNode>,
}

impl BasicContext {
	pub fn temporary(&mut self) -> Variable {
		self.next_temporary += 1;
		Variable(".".into(), self.next_temporary - 1)
	}

	/// Creates an empty component.
	pub fn component(&mut self) -> Component {
		let target = NodeTarget(self.nodes.len());
		self.nodes.insert(target, BasicNode::new());
		Component { entry: target, exit: target }
	}

	/// Pushes a statement on the endpoint of the component.
	pub fn push(&mut self, component: Component, statement: Spanned<Statement>) -> Component {
		let (other, span) = (self.component(), statement.span);
		self[&other.exit].statements.push(statement);
		self.join(component, other, span)
	}

	/// Joins the base component with the target.
	/// Coalesces the base exit block with the target entry block.
	pub fn join(&mut self, mut base: Component, target: Component, span: Span) -> Component {
		let (base_node, target_node) = (&self[&base.exit], &self[&target.entry]);
		if base_node.in_reverse.is_empty() && target_node.in_advance.is_empty() {
			let mut targets = HashMap::new();
			targets.insert(target.entry, base.exit);

			let mut target_node = self.nodes.remove(&target.entry).unwrap();
			target_node.in_reverse.into_iter().for_each(|target|
				self[&target].reverse.node.retarget(&targets));

			let base_node = &mut self[&base.exit];
			base_node.statements.append(&mut target_node.statements);
			base_node.advance = target_node.advance;
			if target.exit != target.entry {
				base.exit = target.exit;
			}
		} else {
			self.link(Direction::Advance, &base.exit, &target.entry, span);
			self.link(Direction::Reverse, &target.entry, &base.exit, span);
			base.exit = target.exit;
		}
		base
	}

	pub fn flatten(self) -> Vec<BasicNode> {
		let targets: HashMap<_, _> = self.nodes.iter().enumerate()
			.map(|(index, (target, _))| (NodeTarget(index), *target)).collect();
		self.nodes.into_iter().map(|(_, mut node)| {
			node.reverse.node.retarget(&targets);
			node.advance.node.retarget(&targets);
			node
		}).collect()
	}

	/// Links the base node to the target node with a jump.
	fn link(&mut self, direction: Direction, base: &NodeTarget, target: &NodeTarget, span: Span) {
		assert_eq!(self[base].branch(direction).node, Branch::Unreachable);
		*self[base].branch(direction) = Spanned::new(Branch::Jump(target.clone()), span);
		self[target].in_edges(direction).push(base.clone());
	}
}

impl Index<&NodeTarget> for BasicContext {
	type Output = BasicNode;

	fn index(&self, index: &NodeTarget) -> &Self::Output {
		self.nodes.get(index).unwrap_or_else(||
			panic!("Node target: {:?}, does not exist in context", index))
	}
}

impl IndexMut<&NodeTarget> for BasicContext {
	fn index_mut(&mut self, index: &NodeTarget) -> &mut Self::Output {
		self.nodes.get_mut(index).unwrap_or_else(||
			panic!("Node target: {:?}, does not exist in context", index))
	}
}
