use std::fmt;

use crate::source::Spanned;

use super::{Direction, FunctionOffset, FunctionTarget, GenericOperation, OperationIdentifier, Token};

pub type Operand<'a> = Spanned<Token<'a>>;

#[derive(Debug)]
pub struct Instruction {
	pub identifier: &'static str,
	pub operation: GenericOperation,
	pub direction: Direction,
	pub polarization: Option<Direction>,
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use super::Direction::*;
		match (self.direction, self.polarization) {
			(Advance, None) => write!(f, "{} {}", self.identifier, self.operation),
			(Advance, Some(Advance)) => write!(f, "+{} {}", self.identifier, self.operation),
			(Advance, Some(Reverse)) => write!(f, "-{} {}", self.identifier, self.operation),
			(Reverse, None) => write!(f, "{}' {}", self.identifier, self.operation),
			(Reverse, Some(Advance)) => write!(f, "+{}' {}", self.identifier, self.operation),
			(Reverse, Some(Reverse)) => write!(f, "-{}' {}", self.identifier, self.operation),
		}
	}
}

/// An index for an instruction within a unit.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InstructionTarget(pub FunctionTarget, pub FunctionOffset);

#[derive(Debug, Clone)]
pub struct TranslationInstruction<'a> {
	pub operation: OperationIdentifier,
	pub operands: Vec<Operand<'a>>,
	pub direction: Direction,
	pub polarization: Option<Direction>,
}
