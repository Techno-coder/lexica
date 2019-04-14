use super::{Comparator, InstructionTarget, LocalTarget, Primitive};
use super::operations::*;

#[derive(Debug)]
pub enum Operation {
	ReversalHint,
	Pass,
	Swap(Swap),
	Add(Add),
	AddImmediate(AddImmediate),
	Minus(Minus),
	MinusImmediate(MinusImmediate),
	Drop(Drop),
	DropImmediate(DropImmediate),
	Restore(Restore),
	Discard(Discard),
	Reset(Reset),
	Call(Call),
	Jump(Jump),
	Branch(Branch),
	BranchImmediate(BranchImmediate),
}
