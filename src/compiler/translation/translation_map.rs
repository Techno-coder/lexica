use std::collections::BTreeMap;
use std::fmt::Write;

use crate::source::{Span, Spanned};

use super::Element;

#[derive(Debug)]
pub struct TranslationMap {
	ranges: BTreeMap<usize, Span>,
	text: String,
}

impl TranslationMap {
	pub fn new(elements: Vec<Spanned<Element>>) -> Self {
		let mut ranges = BTreeMap::new();
		let mut text = String::new();

		for element in elements {
			writeln!(&mut text, "{}", element).unwrap();
			ranges.insert(text.len(), element.span);
		}

		Self { ranges, text }
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	pub fn translate<T>(&self, element: &mut Spanned<T>) {
		element.span = self.translate_span(element.span);
	}

	pub fn translate_span(&self, span: Span) -> Span {
		let lowest = self.highest_span(span.byte_start);
		let highest = self.highest_span(span.byte_end);
		Span::new(lowest.byte_start, highest.byte_end)
	}

	/// Finds the highest source span that overlaps the provided index.
	pub fn highest_span(&self, index: usize) -> Span {
		let mut iterator = self.ranges.range(index..);
		let (_, range) = iterator.next().unwrap();
		*range
	}
}
