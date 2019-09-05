use std::str::CharIndices;

#[derive(Debug)]
pub struct PrefixSplit<'a> {
	characters: CharIndices<'a>,
	string: &'a str,
	split: char,
	end: bool,
}

impl<'a> PrefixSplit<'a> {
	pub fn new(string: &'a str, split: char) -> Self {
		let characters = string.char_indices();
		PrefixSplit { characters, string, split, end: false }
	}
}

impl<'a> Iterator for PrefixSplit<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		match self.characters.next() {
			None => match self.end {
				false => {
					self.end = true;
					Some(self.string)
				}
				true => None,
			}
			Some((index, character)) => match character == self.split {
				true => Some(&self.string[..index]),
				false => self.next()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split() {
		let string = "first.second.third";
		let splits: Vec<_> = PrefixSplit::new(string, '.').collect();
		assert_eq!(splits, &["first", "first.second", "first.second.third"])
	}
}
