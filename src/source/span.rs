use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
	byte_start: usize,
	byte_end: usize,
}

impl Span {
	pub fn new(byte_start: usize, byte_end: usize) -> Span {
		Span { byte_start, byte_end }
	}

	pub fn range(&self) -> Range<usize> {
		self.byte_start..self.byte_end
	}
}

