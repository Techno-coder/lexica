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
}

impl fmt::Display for InterpreterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		use self::InterpreterError::*;
		match self {
			CallStackEmpty => writeln!(f, "Call stack is empty"),
			DropStackEmpty => writeln!(f, "Drop stack is empty"),
			InvalidLocal => writeln!(f, "Local target is out of bounds"),
			NonNumerical => writeln!(f, "Operand is not a numerical type"),
			FloatingCast => writeln!(f, "Invalid cast from floating to integer"),
			SizeIncompatible => writeln!(f, "Byte sizes of operands are incompatible"),
			TypesIncompatible => writeln!(f, "Types of operands are incompatible"),
			InvalidRuntime => writeln!(f, "Runtime construction of operation is invalid"),
			UndefinedComparison => writeln!(f, "Comparison of operands is not invalid"),
			Irreversible => writeln!(f, "Instruction cannot be reversed"),
		}
	}
}
