#[derive(Debug)]
pub enum OperationKey {
	Add,
	AddImmediate,
	Branch,
	BranchImmediate,
	Call,
	Clone,
	Discard,
	Drop,
	DropImmediate,
	Exit,
	Jump,
	Recall,
	Reset,
	Return,
	ReversalHint,
	Swap,
	Minus,
	MinusImmediate,
	Restore,
	Other(&'static str),
}

impl Into<&'static str> for OperationKey {
	fn into(self) -> &'static str {
		match self {
			OperationKey::Add => "add",
			OperationKey::AddImmediate => "add.i",
			OperationKey::Branch => "branch",
			OperationKey::BranchImmediate => "branch.i",
			OperationKey::Call => "call",
			OperationKey::Clone => "clone",
			OperationKey::Discard => "discard",
			OperationKey::Drop => "drop",
			OperationKey::DropImmediate => "drop.i",
			OperationKey::Exit => "exit",
			OperationKey::Jump => "jump",
			OperationKey::Recall => "recall",
			OperationKey::Reset => "reset",
			OperationKey::Return => "return",
			OperationKey::ReversalHint => "*",
			OperationKey::Swap => "swap",
			OperationKey::Minus => "minus",
			OperationKey::MinusImmediate => "minus.i",
			OperationKey::Restore => "restore",
			OperationKey::Other(other) => other,
		}
	}
}
