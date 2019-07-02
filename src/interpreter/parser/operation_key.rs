#[derive(Debug)]
pub enum OperationKey {
	Add,
	AddImmediate,
	Branch,
	BranchImmediate,
	Call,
	Clone,
	Compare,
	Discard,
	Drop,
	DropImmediate,
	Exit,
	Jump,
	Pass,
	Reset,
	Return,
	ReversalHint,
	Swap,
	Minus,
	MinusImmediate,
	Multiply,
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
			OperationKey::Compare => "compare",
			OperationKey::Discard => "discard",
			OperationKey::Drop => "drop",
			OperationKey::DropImmediate => "drop.i",
			OperationKey::Exit => "exit",
			OperationKey::Jump => "jump",
			OperationKey::Pass => "pass",
			OperationKey::Reset => "reset",
			OperationKey::Return => "return",
			OperationKey::ReversalHint => "*",
			OperationKey::Swap => "swap",
			OperationKey::Minus => "minus",
			OperationKey::MinusImmediate => "minus.i",
			OperationKey::Multiply => "multiply",
			OperationKey::Restore => "restore",
			OperationKey::Other(other) => other,
		}
	}
}