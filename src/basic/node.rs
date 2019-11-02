use std::fmt;

use crate::span::{Span, Spanned};

use super::{Branch, Direction, Statement};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeTarget(pub usize);

impl fmt::Display for NodeTarget {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let NodeTarget(target) = self;
		write!(f, "{}", target)
	}
}

#[derive(Debug)]
pub struct BasicNode {
	pub statements: Vec<Spanned<Statement>>,
	pub reverse: Spanned<Branch>,
	pub advance: Spanned<Branch>,
	pub in_reverse: Vec<NodeTarget>,
	pub in_advance: Vec<NodeTarget>,
}

impl BasicNode {
	pub fn new() -> Self {
		BasicNode {
			statements: Vec::new(),
			reverse: Spanned::new(Branch::Unreachable, Span::INTERNAL),
			advance: Spanned::new(Branch::Unreachable, Span::INTERNAL),
			in_reverse: Vec::new(),
			in_advance: Vec::new(),
		}
	}

	pub fn branch(&mut self, direction: Direction) -> &mut Spanned<Branch> {
		match direction {
			Direction::Advance => &mut self.advance,
			Direction::Reverse => &mut self.reverse,
		}
	}

	pub fn in_edges(&mut self, direction: Direction) -> &mut Vec<NodeTarget> {
		match direction {
			Direction::Advance => &mut self.in_advance,
			Direction::Reverse => &mut self.in_reverse,
		}
	}

	pub fn invert(&mut self) {
		std::mem::swap(&mut self.advance, &mut self.reverse);
		std::mem::swap(&mut self.in_advance, &mut self.in_reverse);
		self.statements.reverse();
	}
}

impl fmt::Display for BasicNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "-{}", self.reverse.node)?;
		self.statements.iter().try_for_each(|statement|
			writeln!(f, "{}", statement.node))?;
		write!(f, "+{}", self.advance.node)
	}
}
