use std::collections::HashMap;
use std::fmt::{self, Write};
use std::ops::{Index, IndexMut};

use crate::extension::{Indent, Traverse};
use crate::node::Variable;
use crate::span::{Span, Spanned};

use super::{BasicNode, Branch, Component, Direction, Divergence, NodeTarget,
	Reversibility, Statement};

#[derive(Debug)]
pub struct BasicContext {
	next_temporary: usize,
	next_component: usize,
	reversibility: Reversibility,
	nodes: HashMap<NodeTarget, BasicNode>,
}

impl BasicContext {
	pub fn new(reversibility: Reversibility) -> Self {
		BasicContext {
			next_temporary: 0,
			next_component: 0,
			reversibility,
			nodes: HashMap::new(),
		}
	}

	pub fn is_reversible(&self) -> bool {
		self.reversibility == Reversibility::Reversible
	}

	pub fn temporary(&mut self) -> Variable {
		self.next_temporary += 1;
		Variable::new_temporary(self.next_temporary - 1)
	}

	/// Creates an empty component.
	pub fn component(&mut self) -> Component {
		self.next_component += 1;
		let target = NodeTarget(self.next_component);
		self.nodes.insert(target, BasicNode::new());
		Component::new(target, target)
	}

	/// Reverses a component and the contained nodes and branches.
	pub fn invert(&mut self, component: Component) -> Component {
		Traverse::traverse(component.entry, &mut |traverse, node| {
			self[&node].invert();
			traverse.extend(self[&node].reverse.node.targets().cloned());
			traverse.extend(self[&node].reverse.node.targets().cloned());
		});
		component
	}

	/// Pushes a statement on the endpoint of the component.
	pub fn push(&mut self, component: Component, statement: Spanned<Statement>) -> Component {
		let (other, span) = (self.component(), statement.span);
		self[&other.exit].statements.push(statement);
		self.join(component, other, span)
	}

	/// Joins the base component with the target.
	/// Coalesces the base exit node with the target entry node.
	pub fn join(&mut self, mut base: Component, target: Component, span: Span) -> Component {
		let (base_node, target_node) = (&self[&base.exit], &self[&target.entry]);
		assert_eq!(target_node.reverse.node, Branch::Unreachable);
		assert_eq!(base_node.advance.node, Branch::Unreachable);

		if base_node.in_reverse.is_empty() && target_node.in_advance.is_empty() {
			let mut targets = HashMap::new();
			targets.insert(target.entry, base.exit);

			let mut target_node = self.nodes.remove(&target.entry).unwrap();
			target_node.in_reverse.into_iter().for_each(|target|
				self[&target].retarget(&targets));

			let base_node = &mut self[&base.exit];
			base_node.statements.append(&mut target_node.statements);
			base_node.advance = target_node.advance;
			if target.exit != target.entry {
				base.exit = target.exit;
			}
		} else {
			self.link(Direction::Advance, &base, &target, span);
			self.link(Direction::Reverse, &target, &base, span);
			base.exit = target.exit;
		}
		base
	}

	/// Links the base component endpoint to the target component with a jump.
	pub fn link(&mut self, direction: Direction, base: &Component, target: &Component, span: Span) {
		let (exit, entry) = (base.endpoint(direction), target.endpoint(!direction));
		assert_eq!(self[&exit][direction].node, Branch::Unreachable);
		self[&exit][direction] = Spanned::new(Branch::Jump(entry), span);
		self[&entry].in_edges(direction).push(exit);
	}

	pub fn divergence(&mut self, direction: Direction, base: &Component, divergence: Divergence, span: Span) {
		let exit = base.endpoint(direction);
		divergence.targets().filter(|&target| target != &NodeTarget::UNRESOLVED)
			.for_each(|target| self[target].in_edges(direction).push(exit));
		self[&exit][direction] = Spanned::new(Branch::Divergence(divergence), span);
	}

	/// Replaces unresolved targets in a node with the specified node target.
	pub fn resolve(&mut self, node: &NodeTarget, target: NodeTarget) {
		let mut resolution = |direction| self[node][direction].node.targets()
			.filter(|&target| target == &NodeTarget::UNRESOLVED).last().cloned()
			.map(|_| self[&target].in_edges(direction).push(node.clone()));
		resolution(Direction::Advance);
		resolution(Direction::Reverse);

		let mut targets = HashMap::new();
		targets.insert(NodeTarget::UNRESOLVED, target);
		self[node].retarget(&targets);
	}

	pub fn flatten(mut self, mut component: Component) -> (Vec<BasicNode>, Component) {
		let targets: HashMap<_, _> = self.nodes.iter().enumerate()
			.map(|(index, (target, _))| (*target, NodeTarget(index))).collect();
		component.entry = targets[&component.entry];
		component.exit = targets[&component.exit];

		self.nodes.iter_mut().for_each(|(_, node)| node.retarget(&targets));
		(self.nodes.into_iter().map(|(_, node)| node).collect(), component)
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

impl fmt::Display for BasicContext {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.nodes.iter().try_for_each(|(target, node)| {
			writeln!(f, "{}:", target)?;
			let indent = &mut Indent::new(f);
			writeln!(indent, "{}", node)
		})
	}
}
