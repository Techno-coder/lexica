use super::{Direction, InterpreterError, InterpreterResult, LocalTable, LocalTarget, Primitive};

#[derive(Debug)]
pub struct CallFrame {
	local_table: LocalTable,
	direction: Direction,
}

impl CallFrame {
	pub fn table(&self) -> &LocalTable {
		&self.local_table
	}

	pub fn table_mut(&mut self) -> &mut LocalTable {
		&mut self.local_table
	}
}
