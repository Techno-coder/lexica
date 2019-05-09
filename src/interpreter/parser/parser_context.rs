use crate::source::Spanned;

use super::{Annotation, Element, ParserError, TranslationFunction, TranslationUnit};

/// Stores auxiliary information about the current parser state.
#[derive(Debug, Default)]
pub struct ParserContext<'a> {
	/// Annotations that are yet to be processed.
	pub pending_annotations: Vec<Spanned<Annotation<'a>>>,
	/// The identifier of the current function that is being processed.
	pub pending_function: Option<&'a str>,
	/// The last `Element` that was encountered.
	pub last_element: Option<Spanned<Element<'a>>>,
	/// The errors encountered by the parser.
	pub errors: Vec<Spanned<ParserError<'a>>>,
}

impl<'a> ParserContext<'a> {
	pub fn last_element(&self) -> &Spanned<Element<'a>> {
		self.last_element.as_ref().expect("No element has been parsed")
	}

	pub fn pending_function<'b, 'c>(&self, unit: &'c mut TranslationUnit<'b>)
	                                -> &'c mut TranslationFunction<'b> {
		let identifier = self.pending_function.expect("No function has been reached");
		unit.functions.get_mut(identifier).unwrap()
	}
}
