use std::collections::BTreeMap;

use super::Span;

#[derive(Debug)]
pub struct TextMap {
	text: String,
	line_breaks: BTreeMap<usize, usize>,
}

impl TextMap {
	pub fn new(text: String) -> TextMap {
		let text = "\n".to_owned() + &text;
		let mut line_breaks: BTreeMap<_, _> = text
			.char_indices().filter(|(_, character)| character == &'\n')
			.enumerate().map(|(line, (index, _))| (index, line + 1)).collect();
		line_breaks.insert(0, 1);
		line_breaks.insert(text.len(), line_breaks.len());
		TextMap { text, line_breaks }
	}

	pub fn prefix(&self, span: &Span, height: usize) -> Vec<(usize, &str)> {
		let mut lines = Vec::new();
		let mut line_indexes = self.line_breaks.range(..=span.range().start)
			.rev().take(height + 1).peekable();
		while let Some((byte_index, _)) = line_indexes.next() {
			if let Some((next_byte_index, line_index)) = line_indexes.peek() {
				let substring = &self.text[*next_byte_index + 1..*byte_index];
				lines.push((**line_index, substring));
			}
		}
		lines
	}

	pub fn lines(&self, span: &Span) -> Vec<(usize, &str)> {
		let range = span.range();
		let (start, _) = self.line_breaks.range(..=range.start).next_back().unwrap();
		let (end, _) = self.line_breaks.range(range.end..).next()
			.expect("Span extends past text map");

		let mut lines = Vec::new();
		let mut line_indexes = self.line_breaks.range(start..=end).peekable();
		while let Some((byte_index, line_index)) = line_indexes.next() {
			if let Some((next_byte_index, _)) = line_indexes.peek() {
				let substring = &self.text[*byte_index + 1..**next_byte_index];
				lines.push((*line_index, substring));
			}
		}
		lines
	}

	pub fn suffix(&self, span: &Span, height: usize) -> Vec<(usize, &str)> {
		let mut lines = Vec::new();
		let mut line_indexes = self.line_breaks.range(span.range().end..)
			.take(height + 1).peekable();
		while let Some((byte_index, line_index)) = line_indexes.next() {
			if let Some((next_byte_index, _)) = line_indexes.peek() {
				let substring = &self.text[*byte_index + 1..**next_byte_index];
				lines.push((*line_index, substring));
			}
		}
		lines
	}

	pub fn line_offsets(&self, span: &Span) -> (&str, &str) {
		let range = span.range();
		let mut start = self.line_breaks.range(..=range.start);
		let (start_byte, _) = start.next_back().unwrap();
		(&self.text[*start_byte + 1..range.start], &self.text[range.start..range.end])
	}

	pub fn text(&self) -> &str {
		&self.text
	}
}
