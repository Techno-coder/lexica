use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
	pub byte_start: usize,
	pub byte_end: usize,
}

impl Span {
	pub fn new(byte_start: usize, byte_end: usize) -> Span {
		Span { byte_start, byte_end }
	}

	pub fn range(&self) -> Range<usize> {
		self.byte_start..self.byte_end
	}
}
