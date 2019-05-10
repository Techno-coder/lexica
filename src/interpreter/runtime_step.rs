pub enum RuntimeStep {
	Pass,
	Trapped,
	Halted,
}

impl RuntimeStep {
	pub fn pauses(&self) -> bool {
		use self::RuntimeStep::*;
		match self {
			Trapped => true,
			Halted => true,
			_ => false,
		}
	}
}
