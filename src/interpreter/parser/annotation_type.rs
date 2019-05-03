use std::fmt;

use crate::source::Spanned;

use super::{Annotation, ParserContext, ParserResult, TranslationUnit};

/// An interface for a byte code annotation.
pub trait AnnotationType: fmt::Debug {
	/// Returns the number of arguments this annotation takes.
	fn arity(&self) -> usize;

	/// Modifies the translation unit based on the annotation arguments
	/// and parser context.
	///
	/// The number of arguments provided in the annotation will be guaranteed to be the
	/// same as the `arity` function.
	fn annotate<'a>(&self, annotation: &Spanned<Annotation<'a>>, context: &ParserContext,
	                unit: &mut TranslationUnit) -> ParserResult<'a, ()>;
}

/// Matches a spanned token based on a pattern.
///
/// Returns an error to the surrounding function if it fails.
#[macro_export]
macro_rules! argument {
    ($argument:expr, $pattern:pat, $return:ident) => {
		match &$argument.node {
			$pattern => $return,
			_ => return Err($crate::source::Spanned::new(
					$crate::interpreter::ParserError::UnexpectedArgument($argument.node.clone()),
					$argument.span.clone())),
		};
    };
}
