use std::str::CharIndices;

use super::Span;

#[derive(Debug)]
pub struct SplitWhitespace<'a> {
	text: &'a str,
	iterator: CharIndices<'a>,
}

impl<'a> SplitWhitespace<'a> {
	pub fn new(text: &'a str) -> Self {
		let iterator = text.char_indices();
		Self { text, iterator }
	}

	fn split_comment(&mut self) -> usize {
		self.iterator.find(|(_, character)| character == &'\n')
			.map(|(index, _)| index).unwrap_or(0)
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
		let mut span_start: Option<usize> = None;
		while let Some((index, character)) = self.iterator.next() {
			if character.is_whitespace() {
				if let Some(span_start) = span_start.take() {
					return Some(self.construct_item(span_start, index));
				}
			} else if span_start.is_none() {
				if character == '#' {
					let end_index = self.split_comment();
					return Some(self.construct_item(index, end_index));
				} else {
					span_start = Some(index);
				}
			}
		}
		None
	}
}
