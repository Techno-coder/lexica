use std::iter::Peekable;
use std::str::CharIndices;

use crate::source::SourceKey;
use crate::span::{Span, Spanned};

pub const SINGULARITIES: &[char] = &['\t', '\n', '(', ')', ':'];

/// Splits a source string into spanned string slices.
#[derive(Debug)]
pub struct SourceSplit<'a> {
	string: &'a str,
	characters: Peekable<CharIndices<'a>>,
	source_key: SourceKey,
	byte_end: usize,
}

impl<'a> SourceSplit<'a> {
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		let characters = string.char_indices().peekable();
		SourceSplit { string, characters, source_key, byte_end: string.len() }
	}

	fn slice(&self, byte_start: usize, byte_end: usize) -> Spanned<&'a str> {
		let span = Span::new(self.source_key, byte_start, byte_end);
		Spanned::new(&self.string[byte_start..byte_end], span)
	}
}

impl<'a> Iterator for SourceSplit<'a> {
	type Item = Spanned<&'a str>;

	fn next(&mut self) -> Option<Self::Item> {
		let (byte_start, character) = self.characters.next()?;
		if SINGULARITIES.contains(&character) {
			let byte_end = self.characters.peek()
				.map(|(index, _)| *index).unwrap_or(self.byte_end);
			return Some(self.slice(byte_start, byte_end));
		}

		if character.is_whitespace() {
			return self.next();
		}

		let mut byte_end: Option<usize> = None;
		while let Some((index, character)) = self.characters.peek() {
			if SINGULARITIES.contains(character) || character.is_whitespace() {
				byte_end = Some(*index);
				break;
			} else {
				self.characters.next();
			}
		}

		let byte_end = byte_end.unwrap_or(self.byte_end);
		Some(self.slice(byte_start, byte_end))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_declaration() {
		let string = "\tfn identifier(argument: type):";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["\t", "fn", "identifier", "(", "argument", ":", "type", ")", ":"]);
	}
}
