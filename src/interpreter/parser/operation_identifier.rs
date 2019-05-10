use std::fmt;

/// The identifier each operation matches.
#[derive(Debug, Clone)]
pub enum OperationIdentifier {
	ReversalHint,
	Pass,
	Swap,
	Add,
	AddImmediate,
	Minus,
	MinusImmediate,
	Drop,
	DropImmediate,
	Restore,
	Discard,
	Reset,
	Clone,
	Call,
	Recall,
	Return,
	Exit,
	Jump,
	Branch,
	BranchImmediate,
}

impl OperationIdentifier {
	/// Returns the number of arguments the operation accepts.
	pub fn arity(&self) -> usize {
		use self::OperationIdentifier::*;
		match self {
			ReversalHint => 0,
			Pass => 0,
			Swap => 2,
			Add => 2,
			AddImmediate => 2,
			Minus => 2,
			MinusImmediate => 2,
			Drop => 1,
			DropImmediate => 2,
			Restore => 1,
			Discard => 1,
			Reset => 2,
			Call => 1,
			Recall => 1,
			Return => 0,
			Jump => 1,
			Branch => 4,
			BranchImmediate => 4,
			Clone => 2,
			Exit => 0,
		}
	}

	/// Parses an identifier string into an the operation identifier.
	///
	/// # Errors
	///
	/// Returns `None` if no operation matches the string.
	pub fn parse(identifier: &str) -> Option<OperationIdentifier> {
		use self::OperationIdentifier::*;
		Some(match identifier {
			"*" => ReversalHint,
			"pass" => Pass,
			"swap" => Swap,
			"add" => Add,
			"add.i" => AddImmediate,
			"minus" => Minus,
			"minus.i" => MinusImmediate,
			"drop" => Drop,
			"drop.i" => DropImmediate,
			"restore" => Restore,
			"discard" => Discard,
			"reset" => Reset,
			"clone" => Clone,
			"call" => Call,
			"recall" => Recall,
			"return" => Return,
			"exit" => Exit,
			"jump" => Jump,
			"branch" => Branch,
			"branch.i" => BranchImmediate,
			_ => return None,
		})
	}
}

impl fmt::Display for OperationIdentifier {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::OperationIdentifier::*;
		match self {
			ReversalHint => write!(f, "*"),
			Pass => write!(f, "pass"),
			Swap => write!(f, "swap"),
			Add => write!(f, "add"),
			AddImmediate => write!(f, "add.i"),
			Minus => write!(f, "minus"),
			MinusImmediate => write!(f, "minus.i"),
			Drop => write!(f, "drop"),
			DropImmediate => write!(f, "drop.i"),
			Restore => write!(f, "restore"),
			Discard => write!(f, "discard"),
			Reset => write!(f, "reset"),
			Clone => write!(f, "clone"),
			Call => write!(f, "call"),
			Recall => write!(f, "recall"),
			Return => write!(f, "return"),
			Exit => write!(f, "exit"),
			Jump => write!(f, "jump"),
			Branch => write!(f, "branch"),
			BranchImmediate => write!(f, "branch.i"),
		}
	}
}
