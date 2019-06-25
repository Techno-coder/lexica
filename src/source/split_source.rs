use std::iter::Peekable;
use std::str::CharIndices;

use crate::source::Span;

#[derive(Debug, Clone)]
pub struct SplitSource<'a> {
	text: &'a str,
	iterator: Peekable<CharIndices<'a>>,
}

impl<'a> SplitSource<'a> {
	pub fn new(text: &'a str) -> SplitSource {
		let iterator = text.char_indices().peekable();
		SplitSource { text, iterator }
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

impl<'a> Iterator for SplitSource<'a> {
	type Item = (Span, &'a str);

	fn next(&mut self) -> Option<Self::Item> {
		let mut span_start: Option<usize> = None;
		let mut item_punctuation: Option<bool> = None;

		while let Some((index, character)) = self.iterator.peek() {
			let is_punctuation = Some(is_identifier(*character));
			let text_change = item_punctuation.is_some() && item_punctuation != is_punctuation;

			if let Some(span_start) = span_start {
				if &self.text[span_start..*index] == "//" {
					let end_index = self.split_comment();
					return Some(self.construct_item(span_start, end_index));
				}
			}

			if character.is_whitespace() || text_change {
				if let Some(span_start) = span_start.take() {
					let index = *index;
					return Some(self.construct_item(span_start, index));
				}
			} else if item_punctuation.is_none() {
				item_punctuation = is_punctuation;
				span_start = Some(*index);
			}

			self.iterator.next();
		}
		None
	}
}

pub fn is_identifier(character: char) -> bool {
	character == '_' || !character.is_ascii_punctuation()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split_source() {
		let text = "fn function(n: u32) -> u32 {\n";
		let lexemes: Vec<_> = SplitSource::new(text).map(|(_, lexeme)| lexeme).collect();
		assert_eq!(lexemes, &["fn", "function", "(", "n", ":", "u32", ")", "->", "u32", "{"])
	}

	#[test]
	fn test_comment() {
		let text = "let ~first = 3; // Comment\n";
		let lexemes: Vec<_> = SplitSource::new(text).map(|(_, lexeme)| lexeme).collect();
		assert_eq!(lexemes, &["let", "~", "first", "=", "3", ";", "// Comment"])
	}
}
