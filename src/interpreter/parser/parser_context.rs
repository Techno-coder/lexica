use crate::source::Spanned;

use super::{Annotation, Element};

#[derive(Debug)]
pub struct ParserContext<'a> {
	pub pending_annotations: Vec<Spanned<Annotation<'a>>>,
	pub last_function_label: Option<&'a str>,
	pub last_label: Option<&'a str>,
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
