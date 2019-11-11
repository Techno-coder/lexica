use std::iter::Peekable;
use std::str::CharIndices;

use crate::source::SourceKey;
use crate::span::{Span, Spanned};

pub const SINGULARITIES: &[char] = &['\t', '\n', '(', ')', '~', ',', '*', '.'];

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

	fn comment(&mut self) -> Option<()> {
		loop {
			let (_, character) = self.characters.peek()?;
			match character {
				'\n' => break Some(()),
				_ => self.characters.next(),
			};
		}
	}
}

impl<'a> Iterator for SourceSplit<'a> {
	type Item = Spanned<&'a str>;

	fn next(&mut self) -> Option<Self::Item> {
		let (byte_start, initial) = self.characters.next()?;
		if SINGULARITIES.contains(&initial) {
			let byte_end = self.characters.peek()
				.map(|(index, _)| *index).unwrap_or(self.byte_end);
			return Some(self.slice(byte_start, byte_end));
		}

		if initial.is_whitespace() {
			return self.next();
		} else if let ('/', Some((_, '/'))) = (initial, self.characters.peek()) {
			self.comment()?;
			return self.next();
		}

		let mut byte_end: Option<usize> = None;
		while let Some((index, character)) = self.characters.peek() {
			let class_difference = is_identifier(&initial)
				!= is_identifier(character);
			if SINGULARITIES.contains(character) ||
				character.is_whitespace() || class_difference {
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

fn is_identifier(character: &char) -> bool {
	character == &'_' || !character.is_ascii_punctuation()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_declaration() {
		let string = "\tfn _identifier(argument: type):";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["\t", "fn", "_identifier", "(", "argument", ":", "type", ")", ":"]);
	}

	#[test]
	fn test_path() {
		let string = "root::first::second::third";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["root", "::", "first", "::", "second", "::", "third"]);
	}

	#[test]
	fn test_comment() {
		let string = "// comment\n";
		assert!(SourceSplit::new(string, SourceKey::INTERNAL).next().is_none());
	}
}
