use std::fmt;
use std::ops::Index;

use crate::basic::{BasicBlock, BlockTarget};
use crate::node::{Identifier, Variable};
use crate::source::Spanned;

#[derive(Debug)]
pub struct Function<'a> {
	pub identifier: Identifier<'a>,
	pub parameters: Vec<Spanned<Variable<'a>>>,
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

		write!(f, "{}(", self.identifier)?;
		let split = self.parameters.split_last();
		if let Some((last, rest)) = split {
			rest.iter().try_for_each(|parameter| write!(f, "{}, ", parameter))?;
			write!(f, "{}", last)?;
		}

		writeln!(f, ") {{")?;
		let mut indent = IndentWriter::wrap(f);
		for (index, block) in self.blocks.iter().enumerate() {
			if index == entry_block { write!(indent, "+")?; }
			if index == exit_block { write!(indent, "-")?; }
			writeln!(indent, "{}:", index)?;

			let mut indent = IndentWriter::wrap(&mut indent);
			writeln!(indent, "{}", block)?;
		}

		writeln!(f, "}}")
	}
}

impl<'a> Index<&BlockTarget> for Function<'a> {
	type Output = BasicBlock<'a>;

	fn index(&self, index: &BlockTarget) -> &Self::Output {
		let BlockTarget(index) = index;
		let error = format!("Block target: {}, does not exist in function", index);
		self.blocks.get(*index).expect(&error)
	}
}

