use std::fmt::{Debug, Display};

use super::{Context, Direction};

pub trait NodeConstruct<'a>: Debug + Display {
	/// Nodes that are to be evaluated before this node is executed
	/// These nodes are only evaluated in advance mode
	fn dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency<'a>> { Vec::new() }
	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()>;

	/// Nodes that are to be evaluated before this node is reversed starting from the last
	/// These nodes are only evaluated in reverse mode and the direction should be
	/// as if they were evaluated in advance mode
	fn reverse_dependencies(&'a self, _context: &mut Context<'a>) -> Vec<Dependency<'a>> { Vec::new() }
	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()>;
}

#[derive(Debug, Clone)]
pub struct Dependency<'a> {
	pub node: &'a NodeConstruct<'a>,
	pub direction: Direction,
}

impl<'a> Dependency<'a> {
	/// Specifies the node is to be executed by advancing
	pub fn advance(node: &'a NodeConstruct<'a>) -> Self {
		Self { node, direction: Direction::Advance }
	}

	/// Specifies the node is to be executed in reverse
	pub fn reverse(node: &'a NodeConstruct<'a>) -> Self {
		Self { node, direction: Direction::Reverse }
	}
}

#[derive(Debug)]
pub enum ExecutionStep {
	Void,
	Repeat,
	Value(i64),
}
