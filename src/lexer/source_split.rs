use std::iter::Peekable;
use std::str::CharIndices;

use crate::source::SourceKey;
use crate::span::{Span, Spanned};

pub const SINGULARITIES: &[char] = &['\t', '\n', '(', ')', ',', '*', '.', '$'];

/// Splits a source string into spanned string slices.
#[derive(Debug, Clone)]
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
		if SINGULARITIES.contains(&initial) || initial == '\'' {
			let byte_end = self.characters.peek()
				.map(|(index, _)| *index).unwrap_or(self.byte_end);
			return Some(self.slice(byte_start, byte_end));
		}

		if let ('/', Some((_, '/'))) = (initial, self.characters.peek()) {
			self.comment()?;
			return self.next();
		}

		let mut byte_end: Option<usize> = None;
		while let Some((index, character)) = self.characters.peek() {
			let kind_difference = CharacterKind::kind(initial) !=
				CharacterKind::kind(*character);
			if SINGULARITIES.contains(character) || kind_difference {
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

#[derive(Debug, PartialEq)]
enum CharacterKind {
	Whitespace,
	Identifier,
	Operator,
}

impl CharacterKind {
	pub fn kind(character: char) -> Self {
		match character {
			_ if character.is_whitespace() => CharacterKind::Whitespace,
			_ if !character.is_ascii_punctuation() => CharacterKind::Identifier,
			'_' | '\'' => CharacterKind::Identifier,
			_ => CharacterKind::Operator,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_declaration() {
		let string = "\tfn _identifier(argument: type):";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["\t", "fn", " ", "_identifier",
			"(", "argument", ":", " ", "type", ")", ":"]);
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
		let string = "// comment";
		assert!(SourceSplit::new(string, SourceKey::INTERNAL).next().is_none());
	}

	#[test]
	fn test_prime() {
		let string = "fn function'(variable: &'lifetime)";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["fn", " ", "function'", "(", "variable",
			":", " ", "&", "'", "lifetime", ")"]);
	}

	#[test]
	fn test_unique() {
		let string = "let (~variable) = ~&expression";
		let lexemes: Vec<_> = SourceSplit::new(string, SourceKey::INTERNAL)
			.map(|node| node.node).collect();
		assert_eq!(&lexemes, &["let", " ", "(", "~", "variable", ")",
			" ", "=", " ", "~&", "expression"]);
	}
}
