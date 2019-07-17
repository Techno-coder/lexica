use std::iter::Peekable;
use std::str::CharIndices;

use crate::source::TextMap;

use super::Span;

#[derive(Debug, Clone)]
pub struct SplitSource<'a> {
	text: &'a str,
	iterator: Peekable<CharIndices<'a>>,
	singularities: &'static [char],
	comment: &'static str,
}

impl<'a> SplitSource<'a> {
	pub fn new(text_map: &'a TextMap, singularities: &'static [char], comment: &'static str) -> Self {
		let text = text_map.text();
		let iterator = text.char_indices().peekable();
		Self { text, iterator, singularities, comment }
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
		let mut item_singular: Option<bool> = None;

		while let Some((index, character)) = self.iterator.peek().cloned() {
			if let Some(span_start) = span_start {
				if &self.text[span_start..index] == self.comment {
					let end_index = self.split_comment();
					return Some(self.construct_item(span_start, end_index));
				}
			}

			let is_singular = self.singularities.contains(&character);
			let singularity_change = item_singular == Some(!is_singular);
			let split_change = item_singular == Some(true) || singularity_change;

			if character.is_whitespace() || split_change {
				if let Some(span_start) = span_start.take() {
					return Some(self.construct_item(span_start, index));
				}
			} else if span_start.is_none() {
				span_start = Some(index);
				item_singular = Some(is_singular);
			}

			self.iterator.next();
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_no_singularities() {
		let text = TextMap::new("fn function(n: u32) -> u32 {".to_owned());
		let lexemes: Vec<_> = SplitSource::new(&text, &[], "").map(|(_, lexeme)| lexeme).collect();
		assert_eq!(lexemes, &["fn", "function(n:", "u32)", "->", "u32", "{"]);
	}

	#[test]
	fn test_comment() {
		let text = TextMap::new("let ~first = 3; // Comment".to_owned());
		let lexemes: Vec<_> = SplitSource::new(&text, &[], "//").map(|(_, lexeme)| lexeme).collect();
		assert_eq!(lexemes, &["let", "~first", "=", "3;", "// Comment"])
	}

	#[test]
	fn test_singularities() {
		const SINGULARITIES: &'static [char] = &['(', ')', ';'];
		let text = TextMap::new("let print_result = trace(variable);".to_owned());
		let iterator = SplitSource::new(&text, &SINGULARITIES, "");
		let lexemes: Vec<_> = iterator.map(|(_, lexeme)| lexeme).collect();
		assert_eq!(lexemes, &["let", "print_result", "=", "trace", "(", "variable", ")", ";"]);
	}
}
