use std::fmt;

use crate::source::Spanned;

use super::{Direction, Operation, OperationIdentifier, Token};

#[derive(Debug)]
pub struct Instruction {
	pub operation: Operation,
	pub direction: Direction,
	pub polarization: Option<Direction>,
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use super::Direction::*;
		let identifier = self.operation.identifier();
		match (self.direction, self.polarization) {
			(Advance, None) => write!(f, "{} {}", identifier, self.operation),
			(Advance, Some(Advance)) => write!(f, "+{} {}", identifier, self.operation),
			(Advance, Some(Reverse)) => write!(f, "-{} {}", identifier, self.operation),
			(Reverse, None) => write!(f, "{}' {}", identifier, self.operation),
			(Reverse, Some(Advance)) => write!(f, "+{}' {}", identifier, self.operation),
			(Reverse, Some(Reverse)) => write!(f, "-{}' {}", identifier, self.operation),
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InstructionTarget(pub usize);

#[derive(Debug, Clone)]
pub struct TranslationInstruction<'a> {
	pub operation: OperationIdentifier,
	pub operands: Vec<Spanned<Token<'a>>>,
	pub direction: Direction,
	pub polarization: Option<Direction>,
}
