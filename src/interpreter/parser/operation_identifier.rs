#[derive(Debug)]
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
	Return,
	Jump,
	Branch,
	BranchImmediate,
}

impl OperationIdentifier {
	pub fn argument_count(&self) -> usize {
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
			Return => 0,
			Jump => 1,
			Branch => 4,
			BranchImmediate => 4,
			Clone => 2,
		}
	}

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
			"return" => Return,
			"jump" => Jump,
			"branch" => Branch,
			"branch.i" => BranchImmediate,
			_ => return None,
		})
	}

	pub fn reversible(&self) -> bool {
		use self::OperationIdentifier::*;
		match self {
			ReversalHint => true,
			Pass => true,
			Swap => true,
			Add => true,
			AddImmediate => true,
			Minus => true,
			MinusImmediate => true,
			Drop => true,
			DropImmediate => true,
			Restore => true,
			Discard => false,
			Reset => false,
			Call => true,
			Jump => false,
			Branch => false,
			BranchImmediate => false,
			Return => false,
			Clone => false,
		}
	}
}
