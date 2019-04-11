use std::fmt;

use super::{Context, Dependency, ExecutionStep, NodeConstruct};
use super::{Binding, ConditionalLoop, ExplicitDrop, Mutation, Swap};

#[derive(Debug)]
pub enum Statement<'a> {
	Swap(Swap<'a>),
	Binding(Binding<'a>),
	Mutation(Mutation<'a>),
	ExplicitDrop(ExplicitDrop<'a>),
	ConditionalLoop(ConditionalLoop<'a>),
}

impl<'a> NodeConstruct<'a> for Statement<'a> {
	fn dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency<'a>> {
		match self {
			Statement::Swap(swap) => swap.dependencies(context),
			Statement::Binding(binding) => binding.dependencies(context),
			Statement::Mutation(mutation) => mutation.dependencies(context),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.dependencies(context),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.dependencies(context),
		}
	}

	fn execute(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match self {
			Statement::Swap(swap) => swap.execute(context),
			Statement::Binding(binding) => binding.execute(context),
			Statement::Mutation(mutation) => mutation.execute(context),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.execute(context),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.execute(context),
		}
	}

	fn reverse_dependencies(&'a self, context: &mut Context<'a>) -> Vec<Dependency> {
		match self {
			Statement::Swap(swap) => swap.reverse_dependencies(context),
			Statement::Binding(binding) => binding.reverse_dependencies(context),
			Statement::Mutation(mutation) => mutation.reverse_dependencies(context),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.reverse_dependencies(context),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.reverse_dependencies(context),
		}
	}

	fn reverse(&'a self, context: &mut Context<'a>) -> Result<ExecutionStep, ()> {
		match self {
			Statement::Swap(swap) => swap.reverse(context),
			Statement::Binding(binding) => binding.reverse(context),
			Statement::Mutation(mutation) => mutation.reverse(context),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.reverse(context),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.reverse(context),
		}
	}
}

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
