use crate::span::{Span, Spanned};

use super::{Direction, Branch, Statement};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeTarget(pub usize);

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
}
