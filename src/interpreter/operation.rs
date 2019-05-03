use std::fmt;

use crate::source::Spanned;

use super::{CompilationUnit, Context, InterpreterError,
            InterpreterResult, LocalTable, OperationIdentifier, ParserResult, Token};
use super::operations::*;

pub trait Operation: fmt::Debug + fmt::Display {
	//	fn create(table: &LocalTable, operands: Vec<Spanned<Token>>) -> ParserResult<Box<Self>>;
	fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()>;
	fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		Err(InterpreterError::Irreversible)
	}
}

#[derive(Debug)]
pub enum RefactorOperation {
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
	Recall(Recall),
	Return(Return),
	Exit,
	Jump(Jump),
	Branch(Branch),
	BranchImmediate(BranchImmediate),
}

impl RefactorOperation {
	pub fn identifier(&self) -> OperationIdentifier {
		use super::OperationIdentifier::*;
		match self {
			RefactorOperation::ReversalHint => ReversalHint,
			RefactorOperation::Pass => Pass,
			RefactorOperation::Swap(_) => Swap,
			RefactorOperation::Add(_) => Add,
			RefactorOperation::AddImmediate(_) => AddImmediate,
			RefactorOperation::Minus(_) => Minus,
			RefactorOperation::MinusImmediate(_) => MinusImmediate,
			RefactorOperation::Drop(_) => Drop,
			RefactorOperation::DropImmediate(_) => DropImmediate,
			RefactorOperation::Restore(_) => Restore,
			RefactorOperation::Discard(_) => Discard,
			RefactorOperation::Reset(_) => Reset,
			RefactorOperation::Call(_) => Call,
			RefactorOperation::Recall(_) => Recall,
			RefactorOperation::Return(_) => Return,
			RefactorOperation::Exit => Exit,
			RefactorOperation::Jump(_) => Jump,
			RefactorOperation::Branch(_) => Branch,
			RefactorOperation::BranchImmediate(_) => BranchImmediate,
			RefactorOperation::Clone(_) => Clone,
		}
	}

	pub fn execute(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		Ok(match self {
			RefactorOperation::ReversalHint => (),
			RefactorOperation::Pass => (),
			RefactorOperation::Swap(swap) => swap.execute(context, unit)?,
			RefactorOperation::Add(add) => add.execute(context, unit)?,
			RefactorOperation::AddImmediate(add_immediate) => add_immediate.execute(context, unit)?,
			RefactorOperation::Minus(minus) => minus.execute(context, unit)?,
			RefactorOperation::MinusImmediate(minus_immediate) => minus_immediate.execute(context, unit)?,
			RefactorOperation::Drop(drop) => drop.execute(context, unit)?,
			RefactorOperation::DropImmediate(drop_immediate) => drop_immediate.execute(context, unit)?,
			RefactorOperation::Restore(restore) => restore.execute(context, unit)?,
			RefactorOperation::Discard(discard) => discard.execute(context, unit)?,
			RefactorOperation::Reset(reset) => reset.execute(context, unit)?,
			RefactorOperation::Clone(clone) => clone.execute(context, unit)?,
			RefactorOperation::Call(call) => call.execute(context, unit)?,
			RefactorOperation::Recall(recall) => recall.execute(context, unit)?,
			RefactorOperation::Return(return_operation) => return_operation.execute(context, unit)?,
			RefactorOperation::Exit => (),
			RefactorOperation::Jump(jump) => jump.execute(context, unit)?,
			RefactorOperation::Branch(branch) => branch.execute(context, unit)?,
			RefactorOperation::BranchImmediate(branch_immediate) => branch_immediate.execute(context, unit)?,
		})
	}

	pub fn reverse(&self, context: &mut Context, unit: &CompilationUnit) -> InterpreterResult<()> {
		Ok(match self {
			RefactorOperation::ReversalHint => (),
			RefactorOperation::Pass => (),
			RefactorOperation::Swap(swap) => swap.execute(context, unit)?,
			RefactorOperation::Add(add) => add.reverse(context, unit)?,
			RefactorOperation::AddImmediate(add_immediate) => add_immediate.reverse(context, unit)?,
			RefactorOperation::Minus(minus) => minus.reverse(context, unit)?,
			RefactorOperation::MinusImmediate(minus_immediate) => minus_immediate.reverse(context, unit)?,
			RefactorOperation::Drop(drop) => drop.reverse(context, unit)?,
			RefactorOperation::DropImmediate(drop_immediate) => drop_immediate.reverse(context, unit)?,
			RefactorOperation::Restore(restore) => restore.reverse(context, unit)?,
			RefactorOperation::Call(call) => call.reverse(context, unit)?,
			RefactorOperation::Recall(recall) => recall.reverse(context, unit)?,
			RefactorOperation::Return(return_operation) => return_operation.reverse(context, unit)?,
			RefactorOperation::Exit => (),
			_ => return Err(InterpreterError::Irreversible),
		})
	}
}

impl fmt::Display for RefactorOperation {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			RefactorOperation::ReversalHint => Ok(()),
			RefactorOperation::Pass => Ok(()),
			RefactorOperation::Swap(swap) => write!(f, "{}", swap),
			RefactorOperation::Add(add) => write!(f, "{}", add),
			RefactorOperation::AddImmediate(add_immediate) => write!(f, "{}", add_immediate),
			RefactorOperation::Minus(minus) => write!(f, "{}", minus),
			RefactorOperation::MinusImmediate(minus_immediate) => write!(f, "{}", minus_immediate),
			RefactorOperation::Drop(drop) => write!(f, "{}", drop),
			RefactorOperation::DropImmediate(drop_immediate) => write!(f, "{}", drop_immediate),
			RefactorOperation::Restore(restore) => write!(f, "{}", restore),
			RefactorOperation::Discard(discard) => write!(f, "{}", discard),
			RefactorOperation::Reset(reset) => write!(f, "{}", reset),
			RefactorOperation::Call(call) => write!(f, "{}", call),
			RefactorOperation::Recall(recall) => write!(f, "{}", recall),
			RefactorOperation::Return(return_operation) => write!(f, "{}", return_operation),
			RefactorOperation::Exit => Ok(()),
			RefactorOperation::Jump(jump) => write!(f, "{}", jump),
			RefactorOperation::Branch(branch) => write!(f, "{}", branch),
			RefactorOperation::BranchImmediate(branch_immediate) => write!(f, "{}", branch_immediate),
			RefactorOperation::Clone(clone) => write!(f, "{}", clone),
		}
	}
}
