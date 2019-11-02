use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
pub struct Traverse<T> where T: Eq + Hash {
	visited: HashSet<T>,
	queue: VecDeque<T>,
}

impl<T> Traverse<T> where T: Clone + Eq + Hash {
	pub fn traverse<F>(node: T, function: &mut F) where F: FnMut(&mut Self, T) {
		let mut traverse = Traverse::new(node);
		while let Some(node) = traverse.queue.pop_front() {
			function(&mut traverse, node);
		}
	}

	fn new(node: T) -> Self {
		let mut visited = HashSet::new();
		visited.insert(node.clone());
		let mut queue = VecDeque::new();
		queue.push_back(node);
		Traverse { visited, queue }
	}

	pub fn push(&mut self, node: T) {
		if !self.visited.contains(&node) {
			self.visited.insert(node.clone());
			self.queue.push_back(node);
		}
	}

	pub fn extend<I>(&mut self, iterator: I) where I: Iterator<Item=T> {
		iterator.for_each(|node| self.push(node));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_traverse() {
		let mut elements = Vec::new();
		Traverse::traverse(0, &mut |traverse, node| {
			elements.push(node);
			if node < 7 {
				traverse.push(node);
				traverse.push(node + 1);
			}
		});

		assert_eq!(&elements, &[0, 1, 2, 3, 4, 5, 6, 7]);
	}
}
