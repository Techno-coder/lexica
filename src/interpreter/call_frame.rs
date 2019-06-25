use super::{Direction, Function, InstructionTarget, LocalTable};

#[derive(Debug)]
pub struct CallFrame {
	local_table: LocalTable,
	direction: Direction,
	return_target: InstructionTarget,
}

impl CallFrame {
	pub fn construct(function: &Function, direction: Direction,
	                 return_target: InstructionTarget) -> CallFrame {
		let local_table = function.locals.clone();
		CallFrame { local_table, direction, return_target }
	}

	pub fn table(&self) -> &LocalTable {
		&self.local_table
	}

	pub fn table_mut(&mut self) -> &mut LocalTable {
		&mut self.local_table
	}

	pub fn return_target(&self) -> &InstructionTarget {
		&self.return_target
	}

	pub fn direction(&self) -> Direction {
		self.direction
	}
}
