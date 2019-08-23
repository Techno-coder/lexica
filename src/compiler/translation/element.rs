use std::fmt;

use crate::interpreter::Direction;

#[derive(Debug, Clone)]
pub enum Element {
	Instruction(Instruction),
	Other(String),
}

impl Element {
	pub fn invert(&mut self) {
		if let Element::Instruction(instruction) = self {
			match &mut instruction.polarization {
				Some(polarization) => *polarization = polarization.invert(),
				None => instruction.direction = instruction.direction.invert(),
			}
		}
	}
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

		let operation_index = self.instruction.find(char::is_whitespace)
			.unwrap_or(self.instruction.len());
		let (operation, operands) = self.instruction.split_at(operation_index);

		let direction = match self.direction {
			Direction::Advance => "",
			Direction::Reverse => "'",
		};

		write!(f, "\t{}{}{}{}", polarization, operation, direction, operands)
	}
}
