use crate::source::Spanned;

use super::{Annotation, Element};

/// Stores auxiliary information about the current parser state.
#[derive(Debug)]
pub struct ParserContext<'a> {
	/// Annotations that are yet to be processed.
	pub pending_annotations: Vec<Spanned<Annotation<'a>>>,
	/// The last function label that was encountered.
	pub last_function_label: Option<&'a str>,
	/// The last label that was encountered. This includes function labels.
	pub last_label: Option<&'a str>,
	/// The last `Element` that was encountered.
	pub last_element: Spanned<Element<'a>>,
}

impl<'a> ParserContext<'a> {
	pub fn new(initial_element: Spanned<Element<'a>>) -> Self {
		Self {
			pending_annotations: Vec::new(),
			last_function_label: None,
			last_label: None,
			last_element: initial_element,
		}
	}
}
