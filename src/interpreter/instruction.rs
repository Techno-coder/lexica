use crate::source::Spanned;

use super::{Direction, Operation, Token};

#[derive(Debug)]
pub struct Instruction {
	pub operation: Operation,
	pub direction: Option<Direction>,
	pub polarization: Option<Direction>,
}

#[derive(Debug, Clone)]
pub struct InstructionTarget(pub usize);

#[derive(Debug, Clone)]
pub struct TranslationInstruction<'a> {
	pub operation: &'a str,
	pub arguments: Vec<Spanned<Token<'a>>>,
	pub direction: Option<Direction>,
	pub polarization: Option<Direction>,
}
