use super::{Dependency, Direction};

#[derive(Debug)]
pub struct DependencyBuffer<'a> {
	buffer: Vec<Dependency<'a>>,
	split_index: usize,
	direction: Direction,
}

impl<'a> DependencyBuffer<'a> {
	pub fn new(buffer: Vec<Dependency<'a>>, direction: Direction) -> Self {
		let split_index = match direction {
			Direction::Advance => 0,
			Direction::Reverse => buffer.len() + 1,
		};
		Self { buffer, split_index, direction }
	}

	/// Advances the cursor and returns the next node construct
	/// Cursor holds if changing direction
	pub fn advance(&mut self) -> Option<&Dependency<'a>> {
		match self.direction {
			Direction::Advance => self.split_index += 1,
			Direction::Reverse => self.direction = Direction::Advance
		}
		self.buffer.get(self.split_index - 1)
	}

	/// Advances the cursor in reverse and returns the next node construct
	/// Cursor holds if changing direction
	pub fn reverse(&mut self) -> Option<&Dependency<'a>> {
		if self.direction == Direction::Advance {
			self.direction = Direction::Reverse;
			self.split_index += 1;
		}

		if self.split_index > 1 {
			self.split_index -= 1;
			self.buffer.get(self.split_index - 1)
		} else {
			None
		}
	}
}
