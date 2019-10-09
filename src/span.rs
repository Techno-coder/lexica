use crate::context::Context;
use crate::source::SourceKey;

#[derive(Debug, Copy, Clone)]
pub struct Span {
	pub source: SourceKey,
	pub byte_start: usize,
	pub byte_end: usize,
}

impl Span {
	pub const INTERNAL: Self = Span::new(SourceKey::INTERNAL, 0, 0);

	pub const fn new(source: SourceKey, byte_start: usize, byte_end: usize) -> Self {
		Span { source, byte_start, byte_end }
	}

	pub const fn new_point(source: SourceKey, byte_offset: usize) -> Self {
		Self::new(source, byte_offset, byte_offset)
	}

	pub fn merge(self, other: Span) -> Self {
		self.extend(other.byte_end)
	}

	pub fn extend(mut self, byte_end: usize) -> Self {
		assert!(self.byte_end <= byte_end);
		self.byte_end = byte_end;
		self
	}

	/// Creates a readable representation of the span path and location.
	/// Returns `true` if the span source is internal or not a string.
	pub fn location(&self, context: &Context) -> (String, bool) {
		if self.source == SourceKey::INTERNAL {
			return ("<Internal compiler source>".to_owned(), true);
		}

		let source = self.source.get(context);
		match source.read_string() {
			Err(_) => (format!("{}:<Failure to read source>", source.path.display()), true),
			Ok(string) => {
				let mut line_index = 1;
				let mut last_line_offset = 0;
				let slice = &string[..self.byte_start];
				for (index, character) in slice.char_indices() {
					if character == '\n' {
						line_index += 1;
						last_line_offset = index + 1;
					}
				}

				let character_offset = (self.byte_start + 1) - last_line_offset;
				(format!("{}:{}:{}", source.path.display(), line_index, character_offset), false)
			}
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Spanned<T> {
	pub node: T,
	pub span: Span,
}

impl<T> Spanned<T> {
	pub fn new(node: T, span: Span) -> Self {
		Spanned { node, span }
	}

	pub fn map<F, R>(self, apply: F) -> Spanned<R> where F: FnOnce(T) -> R {
		Spanned::new(apply(self.node), self.span)
	}
}
