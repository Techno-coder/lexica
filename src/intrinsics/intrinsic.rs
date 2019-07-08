use std::fmt;

use crate::node::DataType;

use super::{Context, InterpreterResult};

/// An operation that must be implemented in the host environment.
#[derive(Debug)]
pub struct Intrinsic {
	pub identifier: &'static str,
	pub return_type: DataType<'static>,
	pub parameters: Vec<DataType<'static>>,
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
