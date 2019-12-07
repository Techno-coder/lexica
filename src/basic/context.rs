use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Write};
use std::ops::{Index, IndexMut};

use crate::context::Context;
use crate::extension::{Indent, Traverse};
use crate::node::Variable;
use crate::span::{Span, Spanned};

use super::{BasicNode, Branch, Component, Direction, Divergence, Location,
	NodeTarget, Reversibility, Statement, Value};

type Frame = HashMap<Variable, Span>;

#[derive(Debug)]
pub struct BasicContext<'a> {
	pub context: &'a Context,
	next_temporary: usize,
	next_component: usize,
	pub reversibility: Reversibility,
	nodes: BTreeMap<NodeTarget, BasicNode>,
	frames: Vec<Frame>,
}

impl<'a> BasicContext<'a> {
	pub fn new(context: &'a Context, reversibility: Reversibility) -> Self {
		BasicContext {
			context,
			next_temporary: 0,
			next_component: 0,
			reversibility,
			nodes: BTreeMap::new(),
			frames: vec![Frame::new()],
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
		if let Statement::Binding(variable, _) = &statement.node {
			self.frame().insert(variable.clone(), statement.span);
		}

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
			target_node.in_reverse.iter().for_each(|target|
				self[target].retarget(&targets));

			let base_node = &mut self[&base.exit];
			target_node.in_reverse.iter().for_each(|target|
				base_node.in_reverse.push(*target));

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

	pub fn consume_value(&mut self, value: &Value) {
		if let Value::Location(location) = value {
			self.consume_variable(&location.variable);
		}
	}

	pub fn consume_variable(&mut self, variable: &Variable) {
		self.frames.iter_mut().rev()
			.find(|frame| frame.contains_key(variable))
			.map(|frame| frame.remove(variable));
	}

	pub fn push_frame(&mut self) {
		self.frames.push(Frame::new());
	}

	pub fn pop_frame(&mut self) -> Component {
		let frame = self.frames.pop().expect("Context frame stack is empty");
		match self.is_reversible() {
			false => self.component(),
			true => frame.into_iter().fold(self.component(), |component, (variable, span)| {
				let statement = Statement::ImplicitDrop(Location::new(variable));
				self.push(component, Spanned::new(statement, span))
			})
		}
	}

	fn frame(&mut self) -> &mut Frame {
		self.frames.last_mut().expect("Context frame stack is empty")
	}
}

impl<'a> Index<&NodeTarget> for BasicContext<'a> {
	type Output = BasicNode;

	fn index(&self, index: &NodeTarget) -> &Self::Output {
		self.nodes.get(index).unwrap_or_else(||
			panic!("Node target: {:?}, does not exist in context", index))
	}
}

impl<'a> IndexMut<&NodeTarget> for BasicContext<'a> {
	fn index_mut(&mut self, index: &NodeTarget) -> &mut Self::Output {
		self.nodes.get_mut(index).unwrap_or_else(||
			panic!("Node target: {:?}, does not exist in context", index))
	}
}

impl<'a> fmt::Display for BasicContext<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.nodes.iter().try_for_each(|(target, node)| {
			match node.direction {
				Direction::Advance => writeln!(f, "{}:", target),
				Direction::Reverse => writeln!(f, "!{}:", target),
			}?;

			let indent = &mut Indent::new(f);
			writeln!(indent, "{}", node)
		})
	}
}
