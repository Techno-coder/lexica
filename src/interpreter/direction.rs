#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
	Advance,
	Reverse,
}

impl Direction {
	pub fn compose(&self, other: &Direction) -> Direction {
		if self == other {
			Direction::Advance
		} else {
			Direction::Reverse
		}
	}

	pub fn invert(&self) -> Direction {
		match self {
			Direction::Advance => Direction::Reverse,
			Direction::Reverse => Direction::Advance,
		}
	}
}
