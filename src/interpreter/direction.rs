#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
	Advance,
	Reverse,
}

impl Direction {
	pub fn invert(self) -> Direction {
		match self {
			Direction::Advance => Direction::Reverse,
			Direction::Reverse => Direction::Advance,
		}
	}

	pub fn compose(self, other: Direction) -> Direction {
		match self == other {
			true => Direction::Advance,
			false => Direction::Reverse,
		}
	}
}
