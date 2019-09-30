use std::collections::BTreeMap;

pub trait StringExtension {
	/// Creates a map of line start indexes to line numbers.
	fn line_offsets(&self) -> BTreeMap<usize, usize>;
}

impl StringExtension for &str {
	fn line_offsets(&self) -> BTreeMap<usize, usize> {
		std::iter::once(0).chain(self.char_indices()
			.filter(|(_, character)| character == &'\n')
			.map(|(index, _)| index + 1)).enumerate()
			.map(|(index, line)| (line, index + 1))
			.collect()
	}
}

