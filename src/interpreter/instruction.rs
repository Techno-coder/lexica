use crate::source::Spanned;

use super::{Direction, Operation, OperationIdentifier, Token};

#[derive(Debug)]
pub struct Instruction {
	pub operation: Operation,
	pub direction: Option<Direction>,
	pub polarization: Option<Direction>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct InstructionTarget(pub usize);

#[derive(Debug, Clone)]
pub struct TranslationInstruction<'a> {
	pub operation: OperationIdentifier,
	pub operands: Vec<Spanned<Token<'a>>>,
	pub direction: Option<Direction>,
	pub polarization: Option<Direction>,
}
