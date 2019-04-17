use super::operations::*;
use super::OperationIdentifier;

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
	Clone(Clone),
	Call(Call),
	Return,
	Jump(Jump),
	Branch(Branch),
	BranchImmediate(BranchImmediate),
}

impl Operation {
	pub fn identifier(&self) -> OperationIdentifier {
		use super::OperationIdentifier::*;
		match self {
			Operation::ReversalHint => ReversalHint,
			Operation::Pass => Pass,
			Operation::Swap(_) => Swap,
			Operation::Add(_) => Add,
			Operation::AddImmediate(_) => AddImmediate,
			Operation::Minus(_) => Minus,
			Operation::MinusImmediate(_) => MinusImmediate,
			Operation::Drop(_) => Drop,
			Operation::DropImmediate(_) => DropImmediate,
			Operation::Restore(_) => Restore,
			Operation::Discard(_) => Discard,
			Operation::Reset(_) => Reset,
			Operation::Call(_) => Call,
			Operation::Return => Return,
			Operation::Jump(_) => Jump,
			Operation::Branch(_) => Branch,
			Operation::BranchImmediate(_) => BranchImmediate,
			Operation::Clone(_) => Clone,
		}
	}
}
