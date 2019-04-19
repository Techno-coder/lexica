use std::fmt;

use super::{CompilationUnit, Context, InterpreterError, InterpreterResult, OperationIdentifier};
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
	Clone(CloneLocal),
	Call(Call),
	Return,
	Exit,
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
			Operation::Exit => Exit,
			Operation::Jump(_) => Jump,
			Operation::Branch(_) => Branch,
			Operation::BranchImmediate(_) => BranchImmediate,
			Operation::Clone(_) => Clone,
		}
	}

	pub fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
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
			Operation::Call(call) => call.execute(context, unit),
			Operation::Return => Return::execute(context)?,
			Operation::Exit => (),
			Operation::Jump(jump) => jump.execute(context),
			Operation::Branch(branch) => branch.execute(context)?,
			Operation::BranchImmediate(branch_immediate) => branch_immediate.execute(context)?,
		})
	}

	pub fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		Ok(match self {
			Operation::ReversalHint => (),
			Operation::Pass => (),
			Operation::Swap(swap) => swap.execute(context)?,
			Operation::Add(add) => add.reverse(context)?,
			Operation::AddImmediate(add_immediate) => add_immediate.reverse(context)?,
			Operation::Minus(minus) => minus.reverse(context)?,
			Operation::MinusImmediate(minus_immediate) => minus_immediate.reverse(context)?,
			Operation::Drop(drop) => drop.reverse(context)?,
			Operation::DropImmediate(drop_immediate) => drop_immediate.reverse(context)?,
			Operation::Restore(restore) => restore.reverse(context)?,
			Operation::Call(call) => call.reverse(context, unit)?,
			Operation::Exit => (),
			_ => return Err(InterpreterError::Irreversible),
		})
	}
}

impl fmt::Display for Operation {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Operation::ReversalHint => Ok(()),
			Operation::Pass => Ok(()),
			Operation::Swap(swap) => write!(f, "{}", swap),
			Operation::Add(add) => write!(f, "{}", add),
			Operation::AddImmediate(add_immediate) => write!(f, "{}", add_immediate),
			Operation::Minus(minus) => write!(f, "{}", minus),
			Operation::MinusImmediate(minus_immediate) => write!(f, "{}", minus_immediate),
			Operation::Drop(drop) => write!(f, "{}", drop),
			Operation::DropImmediate(drop_immediate) => write!(f, "{}", drop_immediate),
			Operation::Restore(restore) => write!(f, "{}", restore),
			Operation::Discard(discard) => write!(f, "{}", discard),
			Operation::Reset(reset) => write!(f, "{}", reset),
			Operation::Call(call) => write!(f, "{}", call),
			Operation::Return => Ok(()),
			Operation::Exit => Ok(()),
			Operation::Jump(jump) => write!(f, "{}", jump),
			Operation::Branch(branch) => write!(f, "{}", branch),
			Operation::BranchImmediate(branch_immediate) => write!(f, "{}", branch_immediate),
			Operation::Clone(clone) => write!(f, "{}", clone),
		}
	}
}
