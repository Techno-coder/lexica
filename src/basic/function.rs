use std::fmt;

use crate::basic::{BasicBlock, BlockTarget};

#[derive(Debug)]
pub struct Function<'a> {
	pub entry_block: BlockTarget,
	pub exit_block: BlockTarget,
	pub blocks: Vec<BasicBlock<'a>>,
}

impl<'a> fmt::Display for Function<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::utility::IndentWriter;
		let BlockTarget(entry_block) = self.entry_block;
		let BlockTarget(exit_block) = self.exit_block;

		for (index, block) in self.blocks.iter().enumerate() {
			if index == entry_block { write!(f, "+")?; }
			if index == exit_block { write!(f, "-")?; }
			writeln!(f, "{}:", index)?;

			let mut indent = IndentWriter::wrap(f);
			writeln!(indent, "{}", block)?;
		}
		Ok(())
	}
}
