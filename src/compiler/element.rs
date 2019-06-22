use std::fmt;

use crate::interpreter::Direction;

#[derive(Debug, Clone)]
pub enum Element {
	Instruction(Instruction),
	Other(String),
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Element::Instruction(instruction) => write!(f, "{}", instruction),
			Element::Other(other) => write!(f, "{}", other),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Instruction {
	pub direction: Direction,
	pub polarization: Option<Direction>,
	pub instruction: String,
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let polarization = match self.polarization {
			Some(polarization) => match polarization {
				Direction::Advance => "+",
				Direction::Reverse => "-",
			}
			None => "",
		};

		let direction = match self.direction {
			Direction::Advance => "",
			Direction::Reverse => "'",
		};

		write!(f, "{}{}{}", polarization, self.instruction, direction)
	}
}
