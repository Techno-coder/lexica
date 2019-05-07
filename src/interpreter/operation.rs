use std::fmt;

use crate::source::Span;

use super::{CompilationUnit, Context, InterpreterError, InterpreterResult, Operand,
            ParserContext, ParserResult, TranslationUnit};

pub type GenericOperation = Box<dyn Operation>;

/// An interface that defines how an operation is executed or reversed.
pub trait Operation: fmt::Debug + fmt::Display {
	/// Executes the operation on the given context by advancing.
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()>;
	/// Reverses the operation on the given context.
	fn reverse(&self, _context: &mut Context, _unit: &CompilationUnit) -> InterpreterResult<()> {
		Err(InterpreterError::Irreversible)
	}
}

/// An interface that defines how an operation is constructed.
pub trait Operational: fmt::Debug + fmt::Display {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation>;
}
