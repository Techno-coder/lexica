use std::collections::BTreeMap;

use super::Span;

#[derive(Debug)]
pub struct TextMap {
	text: String,
	/// Stores a mapping from line break byte indexes to one indexed line numbers.
	line_breaks: BTreeMap<usize, usize>,
}

impl TextMap {
	pub fn new(text: String) -> TextMap {
		let text = "\n".to_owned() + &text + "\n";
		let mut line_breaks: BTreeMap<_, _> = text
			.char_indices().filter(|(_, character)| character == &'\n')
			.enumerate().map(|(line, (index, _))| (index, line + 1)).collect();
		line_breaks.insert(0, 1);
		line_breaks.insert(text.len(), line_breaks.len());
		TextMap { text, line_breaks }
	}

	/// Returns an amount of lines before the span with the line number.
	pub fn prefix(&self, span: Span, height: usize) -> Vec<(usize, &str)> {
		let span = offset_span(span);
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

	/// Returns all the lines overlapping the span with the line number.
	pub fn lines(&self, span: Span) -> Vec<(usize, &str)> {
		let span = offset_span(span);
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

	/// Returns an amount of lines after the span with the line number.
	pub fn suffix(&self, span: Span, height: usize) -> Vec<(usize, &str)> {
		let span = offset_span(span);
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

	/// Calculates the line prefix of the span with the span text itself.
	pub fn line_offsets(&self, span: Span) -> (&str, &str) {
		let span = offset_span(span);
		let range = span.range();

		let mut start = self.line_breaks.range(..=range.start);
		let (start_byte, _) = start.next_back().unwrap();
		let prefix_start = usize::min(*start_byte + 1, range.start);
		(&self.text[prefix_start..range.start], &self.text[range])
	}

	/// Returns the text stored in the TextMap.
	/// Guarantees the string returned has an ending line break.
	pub fn text(&self) -> &str {
		&self.text[1..]
	}
}

/// Span offset compensates for initial line break in text map.
fn offset_span(mut span: Span) -> Span {
	span.byte_start += 1;
	span.byte_end += 1;
	span
}
