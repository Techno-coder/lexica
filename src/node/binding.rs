use std::fmt;

use super::{Expression, Variable};
use super::NodeConstruct;

#[derive(Debug)]
pub struct Binding<'a> {
	pub variable: Variable<'a>,
	pub expression: Expression<'a>,
}

impl<'a> NodeConstruct<'a> for Binding<'a> {}

impl<'a> fmt::Display for Binding<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "let {} = {}", self.variable, self.expression)
	}
}
