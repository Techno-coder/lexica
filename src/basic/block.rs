use std::fmt;

use crate::interpreter::Direction;
use crate::source::Spanned;

use super::{Branch, Statement};

#[derive(Debug)]
pub struct BasicBlock<'a> {
	pub advance: Spanned<Branch<'a>>,
	pub reverse: Spanned<Branch<'a>>,
	pub statements: Vec<Spanned<Statement<'a>>>,

	/// Determines whether the block has been inverted or not.
	pub direction: Direction,
	pub in_advance: Vec<BlockTarget>,
	pub in_reverse: Vec<BlockTarget>,
}

impl<'a> BasicBlock<'a> {
	pub fn new_single(statement: Spanned<Statement<'a>>) -> Self {
		let mut block = Self::default();
		block.statements.push(statement);
		block
	}

	pub fn invert(&mut self) {
		std::mem::swap(&mut self.advance, &mut self.reverse);
		std::mem::swap(&mut self.in_advance, &mut self.in_reverse);
		self.direction = self.direction.invert();
		self.statements.reverse();
	}
}

impl<'a> Default for BasicBlock<'a> {
	fn default() -> Self {
		use crate::source::Span;
		Self {
			advance: Spanned::new(Branch::default(), Span::SENTINEL),
			reverse: Spanned::new(Branch::default(), Span::SENTINEL),
			statements: Vec::new(),
			direction: Direction::Advance,
			in_advance: Vec::new(),
			in_reverse: Vec::new(),
		}
	}
}

impl<'a> fmt::Display for BasicBlock<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "-{};", self.reverse)?;
		self.statements.iter().try_for_each(|statement| writeln!(f, "{};", statement))?;
		write!(f, "+{};", self.advance)
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BlockTarget(pub usize);

impl BlockTarget {
	pub const SENTINEL: Self = BlockTarget(std::usize::MAX);
}

impl fmt::Display for BlockTarget {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let BlockTarget(target) = self;
		match self == &Self::SENTINEL {
			false => write!(f, "{}", target),
			true => write!(f, "<?>"),
		}
	}
}
