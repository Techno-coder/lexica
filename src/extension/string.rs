use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;

pub type LineOffsets = BTreeMap<LineOffset, usize>;

/// Represents an index to the start of a line.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LineOffset(pub usize);

impl Borrow<usize> for LineOffset {
	fn borrow(&self) -> &usize {
		self.deref()
	}
}

impl Deref for LineOffset {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		let LineOffset(byte_offset) = self;
		byte_offset
	}
}

impl fmt::Debug for LineOffset {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "LineOffset({})", self.deref())
	}
}

pub trait StringExtension {
	/// Creates a map of line start indexes to line numbers.
	fn line_offsets(&self) -> LineOffsets;
	/// Checks if another string is a prefix or equal to this string.
	fn prefix_equal(&self, other: &str) -> bool;
}

impl StringExtension for &str {
	fn line_offsets(&self) -> LineOffsets {
		std::iter::once(0).chain(self.char_indices()
			.filter(|(_, character)| character == &'\n')
			.map(|(offset, _)| offset + 1)).enumerate()
			.map(|(line, offset)| (LineOffset(offset), line + 1))
			.collect()
	}

	fn prefix_equal(&self, other: &str) -> bool {
		self == &other || self.starts_with(other)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_prefix_equal() {
		assert!("context".prefix_equal("con"));
		assert!("context".prefix_equal("context"));
		assert!(!"context".prefix_equal("context_"));
	}
}
