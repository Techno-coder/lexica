use std::fmt;
use std::marker::PhantomData;

use crate::source::Span;

use super::{CompilationUnit, Context, GenericOperation, InterpreterError, InterpreterResult, Operand, Operation,
            Operational, ParserContext, ParserResult, Reversible, TranslationUnit};

#[derive(Debug)]
pub struct Reverser<T> {
	operation: GenericOperation,
	operational: PhantomData<T>,
}

impl<T> Operational for Reverser<T> where T: Operational + 'static {
	fn parse<'a>(span: &Span, operands: &Vec<Operand<'a>>, context: &ParserContext,
	             unit: &TranslationUnit) -> ParserResult<'a, GenericOperation> {
		let operation = T::parse(span, operands, context, unit)?;
		Ok(Box::new(Self { operation, operational: PhantomData }))
	}
}

impl<T> Operation for Reverser<T> where T: Operational {
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		match self.operation.reversible() {
			Some(reversible) => reversible.reverse(context, unit),
			None => Err(InterpreterError::Irreversible),
		}
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
