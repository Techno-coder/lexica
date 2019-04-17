use std::fmt;

use super::{Binding, ConditionalLoop, ExplicitDrop, Mutation, Swap};
use super::NodeConstruct;

#[derive(Debug)]
pub enum Statement<'a> {
	Swap(Swap<'a>),
	Binding(Binding<'a>),
	Mutation(Mutation<'a>),
	ExplicitDrop(ExplicitDrop<'a>),
	ConditionalLoop(ConditionalLoop<'a>),
}

impl<'a> NodeConstruct<'a> for Statement<'a> {}

impl<'a> fmt::Display for Statement<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Statement::Swap(swap) => write!(f, "{};", swap),
			Statement::Binding(binding) => write!(f, "{};", binding),
			Statement::Mutation(mutation) => write!(f, "{};", mutation),
			Statement::ExplicitDrop(explicit_drop) => write!(f, "{};", explicit_drop),
			Statement::ConditionalLoop(conditional_loop) => write!(f, "{}", conditional_loop),
		}
	}
}
