use std::fmt;

use hashbrown::HashMap;

use crate::source::Spanned;

use super::{Function, Identifier, NodeConstruct, NodeVisitor, Structure};

#[derive(Debug)]
pub struct SyntaxUnit<'a> {
	pub structures: HashMap<Identifier<'a>, Spanned<Structure<'a>>>,
	pub functions: HashMap<Identifier<'a>, Spanned<Function<'a>>>,
}

impl<'a> NodeConstruct<'a> for Spanned<SyntaxUnit<'a>> {
	fn accept<V: NodeVisitor<'a>>(&mut self, visitor: &mut V) -> V::Result {
		visitor.syntax_unit(self)
	}
}

impl<'a> fmt::Display for SyntaxUnit<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.structures.values().try_for_each(|structure| writeln!(f, "{}\n", structure))?;
		self.functions.values().try_for_each(|function| writeln!(f, "{}\n", function))
	}
}

/// Applies a function to all constructs within the unit.
macro_rules! apply_unit {
    ($unit:expr, $transformer:block, $variable:ident) => {
        $unit.structures.values_mut().for_each(|$variable| $transformer);
        $unit.functions.values_mut().for_each(|$variable| $transformer);
    };
}
