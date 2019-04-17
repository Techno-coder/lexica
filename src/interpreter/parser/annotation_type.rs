use std::fmt;

use crate::source::Spanned;

use super::{Annotation, ParserContext, ParserResult, TranslationUnit};

pub trait AnnotationType: fmt::Debug {
	fn argument_count(&self) -> usize;
	fn annotate<'a>(&self, annotation: &'a Spanned<Annotation>, context: &'a ParserContext,
	                unit: &mut TranslationUnit) -> ParserResult<'a, ()>;
}

macro_rules! argument {
    ($argument:expr, $pattern:pat, $return:ident) => {
		match &$argument.node {
			$pattern => $return,
			_ => return Err($crate::source::Spanned::new(
					$crate::interpreter::ParserError::UnexpectedArgument(&$argument.node),
					$argument.span.clone())),
		};
    };
}
