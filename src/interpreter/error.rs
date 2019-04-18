use std::fmt;

pub type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Debug, Clone)]
pub enum InterpreterError {
	CallStackEmpty,
	DropStackEmpty,
	InvalidLocal,
	NonNumerical,
	FloatingCast,
	SizeIncompatible,
	TypesIncompatible,
	InvalidRuntime,
	UndefinedComparison,
	Irreversible,
	MissingEntryPoint,
	NextInstructionNull,
}

impl fmt::Display for InterpreterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use self::InterpreterError::*;
		match self {
			CallStackEmpty => write!(f, "Call stack is empty"),
			DropStackEmpty => write!(f, "Drop stack is empty"),
			InvalidLocal => write!(f, "Local target is out of bounds"),
			NonNumerical => write!(f, "Operand is not a numerical type"),
			FloatingCast => write!(f, "Invalid cast from floating to integer"),
			SizeIncompatible => write!(f, "Byte sizes of operands are incompatible"),
			TypesIncompatible => write!(f, "Types of operands are incompatible"),
			InvalidRuntime => write!(f, "Runtime construction of operation is invalid"),
			UndefinedComparison => write!(f, "Comparison of operands is not invalid"),
			Irreversible => write!(f, "Instruction cannot be reversed"),
			MissingEntryPoint => write!(f, "Compilation unit has no entry point"),
			NextInstructionNull => write!(f, "Context has no instruction to advance"),
		}
	}
}
