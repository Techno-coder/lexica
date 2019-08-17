use std::fmt;

use hashbrown::HashMap;

use crate::source::Spanned;

use super::{BlockTarget, Expression};

pub type Mapping = HashMap<BlockTarget, BlockTarget>;

#[derive(Debug)]
pub enum Branch<'a> {
	Return(Spanned<Expression<'a>>),
	Conditional(ConditionalBranch<'a>),
	Jump(BlockTarget),
}

impl<'a> Branch<'a> {
	pub const SENTINEL: Self = Branch::Jump(BlockTarget::SENTINEL);

	pub fn replace(&mut self, mapping: &Mapping) {
		match self {
			Branch::Return(_) => (),
			Branch::Conditional(branch) => branch.replace(mapping),
			Branch::Jump(target) => {
				assert_ne!(target, &BlockTarget::SENTINEL);
				*target = mapping[target].clone()
			}
		}
	}
}

impl<'a> fmt::Display for Branch<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Branch::Return(branch_return) => write!(f, "return {}", branch_return),
			Branch::Conditional(conditional) => write!(f, "{}", conditional),
			Branch::Jump(target) => write!(f, "jump {}", target),
		}
	}
}

impl<'a> Default for Branch<'a> {
	fn default() -> Self {
		Self::SENTINEL
	}
}

#[derive(Debug)]
pub struct ConditionalBranch<'a> {
	pub condition: Spanned<Expression<'a>>,
	pub target: BlockTarget,
	pub default: BlockTarget,
}

impl<'a> ConditionalBranch<'a> {
	pub fn replace(&mut self, mapping: &Mapping) {
		self.target = mapping.get(&self.target).cloned()
			.unwrap_or(self.target.clone());
		self.default = mapping.get(&self.default).cloned()
			.unwrap_or(self.default.clone());
	}
}

impl<'a> fmt::Display for ConditionalBranch<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "branch {} if {} else {}", self.target, self.condition, self.default)
	}
}
