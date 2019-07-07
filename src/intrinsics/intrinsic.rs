use std::fmt;

use crate::interpreter::Size;

use super::{Context, InterpreterResult};

/// An operation that must be implemented in the host environment.
#[derive(Debug)]
pub struct Intrinsic {
	pub identifier: &'static str,
	pub return_type: Size,
	pub parameters: Vec<Size>,
	pub function: IntrinsicFunction,
}

#[derive(Clone)]
pub struct IntrinsicFunction(pub fn(&mut Context) -> InterpreterResult<()>);

impl fmt::Debug for IntrinsicFunction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let IntrinsicFunction(function) = self;
		write!(f, "IntrinsicFunction({:p})", function)
	}
}
