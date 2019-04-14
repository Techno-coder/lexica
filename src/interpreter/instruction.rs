use super::{Direction, Operation};

#[derive(Debug)]
pub struct Instruction {
	pub operation: Operation,
	pub direction: Option<Direction>,
	pub polarization: Option<Direction>,
}

#[derive(Debug, Clone)]
pub struct InstructionTarget(pub usize);
