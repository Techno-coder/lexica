use super::{Comparator, Context, InstructionTarget, InterpreterResult, LocalTable, LocalTarget,
            Primitive};

#[derive(Debug)]
pub struct Branch {
	comparator: Comparator,
	left: LocalTarget,
	right: LocalTarget,
	target: InstructionTarget,
}

impl Branch {
	pub fn new(table: &LocalTable, comparator: Comparator, left: LocalTarget, right: LocalTarget,
	           target: InstructionTarget) -> InterpreterResult<Branch> {
		let left_local = table.local(&left)?;
		let right_local = table.local(&right)?;
		let _comparison = comparator.compare(left_local, right_local)?;
		Ok(Branch { comparator, left, right, target })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		let table = context.frame()?.table();
		let left_local = &table[&self.left];
		let right_local = &table[&self.right];
		let comparison = self.comparator.compare(left_local, right_local)?;
		if comparison == true {
			context.set_next_instruction(&self.target);
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct BranchImmediate {
	comparator: Comparator,
	local: LocalTarget,
	immediate: Primitive,
	target: InstructionTarget,
}

impl BranchImmediate {
	pub fn new(table: &LocalTable, comparator: Comparator, local: LocalTarget, immediate: Primitive,
	           target: InstructionTarget) -> InterpreterResult<BranchImmediate> {
		let table_local = table.local(&local)?;
		let _comparison = comparator.compare(table_local, &immediate)?;
		Ok(BranchImmediate { comparator, local, immediate, target })
	}

	pub fn execute(&self, context: &mut Context) -> InterpreterResult<()> {
		let table = context.frame()?.table();
		let local = &table[&self.local];
		let comparison = self.comparator.compare(local, &self.immediate)?;
		if comparison == true {
			context.set_next_instruction(&self.target);
		}
		Ok(())
	}
}
