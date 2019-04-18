use std::str::CharIndices;

use super::Span;

#[derive(Debug)]
pub struct SplitWhitespace<'a> {
	text: &'a str,
	iterator: CharIndices<'a>,
	span_start: Option<usize>,
}

impl<'a> SplitWhitespace<'a> {
	pub fn new(text: &'a str) -> Self {
		let iterator = text.char_indices();
		Self {
			text,
			iterator,
			span_start: None,
		}
	}

	fn split_comment(&mut self) -> usize {
		self.iterator.find(|(_, character)| character == &'\n')
		    .map(|(index, _)| index)
		    .unwrap_or(0)
	}

	fn construct_item(&self, span_start: usize, span_end: usize) -> (Span, &'a str) {
		let span = Span::new(span_start, span_end);
		let string = &self.text[span_start..span_end];
		(span, string)
	}
}

impl<'a> Iterator for SplitWhitespace<'a> {
	type Item = (Span, &'a str);

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((index, character)) = self.iterator.next() {
			if character.is_whitespace() {
				if let Some(span_start) = self.span_start {
					self.span_start = None;
					let item = self.construct_item(span_start, index);
					return Some(item);
				}
			} else if self.span_start.is_none() {
				if character == '#' {
					let end_index = self.split_comment();
					let item = self.construct_item(index, end_index);
					return Some(item);
				} else {
					self.span_start = Some(index);
				}
			}
		}
		None
	}
}
