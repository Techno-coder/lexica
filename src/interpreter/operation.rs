use super::{Context, OperationIdentifier};
use super::operations::*;
use crate::interpreter::error::InterpreterResult;

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
	Clone(CloneLocal),
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

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		Ok(match self {
			Operation::ReversalHint => (),
			Operation::Pass => (),
			Operation::Swap(swap) => swap.execute(context)?,
			Operation::Add(add) => add.execute(context)?,
			Operation::AddImmediate(add_immediate) => add_immediate.execute(context)?,
			Operation::Minus(minus) => minus.execute(context)?,
			Operation::MinusImmediate(minus_immediate) => minus_immediate.execute(context)?,
			Operation::Drop(drop) => drop.execute(context)?,
			Operation::DropImmediate(drop_immediate) => drop_immediate.execute(context)?,
			Operation::Restore(restore) => restore.execute(context)?,
			Operation::Discard(discard) => discard.execute(context)?,
			Operation::Reset(reset) => reset.execute(context)?,
			Operation::Clone(clone) => clone.execute(context)?,
			Operation::Call(call) => call.execute(context),
			Operation::Return => (), // TODO
			Operation::Jump(jump) => jump.execute(context),
			Operation::Branch(branch) => branch.execute(context)?,
			Operation::BranchImmediate(branch_immediate) => branch_immediate.execute(context)?,
		})
	}
}
