use crate::source::{Span, Spanned};

use super::{Comparator, CompileContext, Float, FunctionOffset, Integer, InterpreterResult,
            LocalTable, LocalTarget, Operand, ParserError, ParserResult, Primitive, Size,
            Token, TranslationFunction};

/// Gets the last defined function from the current context.
pub fn base_function<'a>(context: CompileContext<'a>, span: &Span)
                         -> ParserResult<'a, &'a TranslationFunction<'a>> {
	context.pending_function.ok_or(Spanned::new(ParserError::FunctionMissingContext, span.clone()))
}

/// Gets the local table from a potentially erroneous function.
pub fn local_table<'a, 'b>(function: &ParserResult<'a, &'b TranslationFunction>)
                           -> ParserResult<'a, &'b LocalTable> {
	function.clone().map(|function| &function.locals)
}

/// Transforms an `InterpreterResult` into a `ParserResult`.
pub fn error<'a, T>(result: InterpreterResult<T>, span: &Span) -> ParserResult<'a, T> {
	result.map_err(|error| Spanned::new(ParserError::Interpreter(error), span.clone()))
}

/// Transforms the operand into a `LocalTarget`.
pub fn local<'a>(local: &Operand<'a>) -> ParserResult<'a, LocalTarget> {
	match local.node {
		Token::UnsignedInteger(integer) => Ok(LocalTarget(integer as usize)),
		_ => Err(local.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

/// Transforms the operand into a `Primitive`.
pub fn primitive<'a>(primitive: &Operand<'a>) -> ParserResult<'a, Primitive> {
	Ok(match primitive.node {
		Token::UnsignedInteger(integer) => Primitive::Integer(Integer::Unsigned64(integer)),
		Token::SignedInteger(integer) => Primitive::Integer(Integer::Signed64(integer)),
		Token::Float(float) => Primitive::Float(Float::Float64(float)),
		_ => return Err(primitive.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

/// Transforms the operand into a `Size`.
pub fn size<'a>(size: &Operand<'a>) -> ParserResult<'a, Size> {
	match size.node {
		Token::Identifier(identifier) => {
			Size::parse(identifier).map_err(|error| Spanned::new(error, size.span.clone()))
		}
		_ => Err(size.map(|token| ParserError::UnexpectedOperand(token.clone())))
	}
}

/// Transforms the operand into a label target string.
pub fn target<'a>(target: &Operand<'a>) -> ParserResult<'a, String> {
	Ok(match target.node {
		Token::Identifier(identifier) => identifier.to_owned(),
		Token::UnsignedInteger(integer) => integer.to_string(),
		Token::SignedInteger(integer) => integer.to_string(),
		_ => return Err(target.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}

/// Transforms the operand into a `FunctionOffset`.
pub fn target_label<'a>(span: &Span, target_label: &Operand<'a>, function: &TranslationFunction)
                        -> ParserResult<'a, FunctionOffset> {
	let target = target(target_label)?;
	function.labels.get(&target).cloned()
		.ok_or(Spanned::new(ParserError::UndefinedLabel(target), span.clone()))
}

/// Transforms the operand into a `Comparator`.
pub fn comparator<'a>(comparator: &Operand<'a>) -> ParserResult<'a, Comparator> {
	Ok(match comparator.node {
		Token::Equal => Comparator::Equal,
		Token::LessThan => Comparator::LessThan,
		Token::LessThanEqual => Comparator::LessThanEqual,
		Token::GreaterThan => Comparator::GreaterThan,
		Token::GreaterThanEqual => Comparator::GreaterThanEqual,
		_ => return Err(comparator.map(|token| ParserError::UnexpectedOperand(token.clone())))
	})
}
