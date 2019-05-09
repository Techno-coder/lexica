use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, CompileContext, CompileResult, Context, InterpreterResult, Operand};

pub type GenericOperation = Box<dyn Operation>;

/// An interface that defines how an operation is executed.
pub trait Operation: fmt::Debug + fmt::Display {
	/// Executes the operation on the given context by advancing.
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()>;
	/// Provides a reversible variant of the operation.
	fn reversible(&self) -> Option<&Reversible> { None }
}

/// An interface that defines how an operation is reversed.
pub trait Reversible: Operation {
	/// Reverses the operation on the given context.
	fn reverse(&self, _context: &mut Context, _unit: &CompilationUnit) -> InterpreterResult<()>;
}

/// An interface that defines how an operation is constructed.
pub trait Operational: fmt::Debug + fmt::Display {
	fn compile<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &CompileContext)
	               -> CompileResult<'a, GenericOperation>;
}
