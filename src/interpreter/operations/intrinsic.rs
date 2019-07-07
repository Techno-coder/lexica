use std::fmt;

use crate::intrinsics::IntrinsicFunction;

use super::{CompilationUnit, CompileContext, Context, InterpreterResult, Operation};

/// A wrapper over an intrinsic operation.
/// This operation cannot be directly constructed and is instead returned
/// from the compilation of the Call operation.
pub struct Intrinsic {
	identifier: &'static str,
	function: IntrinsicFunction,
}

impl Intrinsic {
	pub fn construct(identifier: &str, context: &CompileContext) -> Option<Intrinsic> {
		let intrinsic = context.intrinsics.get(identifier)?;
		let function = intrinsic.function.clone();
		Some(Intrinsic { identifier: intrinsic.identifier, function })
	}
}

impl Operation for Intrinsic {
	fn execute(&self, context: &mut Context, _: &CompilationUnit) -> InterpreterResult<()> {
		let IntrinsicFunction(function) = self.function;
		function(context)
	}
}

impl fmt::Debug for Intrinsic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.identifier)
	}
}

impl fmt::Display for Intrinsic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.identifier)
	}
}
