use std::fmt;
use std::marker::PhantomData;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, GenericOperation,
            InterpreterResult, Operand, Operation, Operational, Reversible};

#[derive(Debug)]
pub struct Reverser<T> {
	operation: GenericOperation,
	operational: PhantomData<T>,
}

impl<T> Operational for Reverser<T> where T: Operational + 'static {
	fn arity() -> usize { T::arity() }

	fn compile<'a, 'b>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext<'a, 'b>)
	                   -> CompileResult<'a, GenericOperation> {
		let operation = T::compile(span, operands, context)?;
		match operation.reversible().is_some() {
			true => Ok(Box::new(Self { operation, operational: PhantomData })),
			false => panic!("Reverser cannot be applied on irreversible operations"),
		}
	}
}

impl<T> Operation for Reverser<T> where T: Operational {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.operation.reversible().unwrap().reverse(context, unit)
	}

	fn reversible(&self) -> Option<&Reversible> {
		Some(self)
	}
}

impl<T> Reversible for Reverser<T> where T: Operational {
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		self.operation.execute(context, unit)
	}
}

impl<T> fmt::Display for Reverser<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.operation)
	}
}
