use super::Span;

#[derive(Debug)]
pub struct TextMap {
	text: String,
}

impl TextMap {
	pub fn new(text: String) -> TextMap {
		TextMap { text }
	}

	pub fn apply(&self, span: &Span) -> &str {
		&self.text[span.range()]
	}
}
