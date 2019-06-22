use std::fmt::{Debug, Display};

use super::NodeVisitor;

pub trait NodeConstruct<'a>: Debug + Display {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result;
}
