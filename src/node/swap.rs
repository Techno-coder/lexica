use std::fmt;

use super::{Identifier, NodeConstruct};

#[derive(Debug)]
pub struct Swap<'a> {
	pub left: Identifier<'a>,
	pub right: Identifier<'a>,
}

impl<'a> NodeConstruct<'a> for Swap<'a> {}

impl<'a> fmt::Display for Swap<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "{} <=> {}", self.left, self.right)
	}
}
