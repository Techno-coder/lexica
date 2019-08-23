use std::fmt;
use std::ops::{Index, IndexMut};

use hashbrown::HashMap;

use crate::basic::{BasicBlock, BlockTarget, Branch, ConditionalBranch, Function, Statement};
use crate::source::{Span, Spanned};

#[derive(Debug)]
pub struct Component<'a> {
	/// The exit block of the component when advancing.
	pub advance_block: BlockTarget,
	/// The exit block of the component when reversing.
	pub reverse_block: BlockTarget,
	pub blocks: HashMap<BlockTarget, BasicBlock<'a>>,
}

impl<'a> Component<'a> {
	pub fn new_empty(target: BlockTarget) -> Self {
		Self::new_single(target, BasicBlock::default())
	}

	pub fn new_single(target: BlockTarget, block: BasicBlock<'a>) -> Self {
		let mut blocks = HashMap::new();
		blocks.insert(target.clone(), block);

		Self {
			advance_block: target.clone(),
			reverse_block: target.clone(),
			blocks,
		}
	}

	pub fn new_paired(entry_target: BlockTarget, exit_target: BlockTarget) -> Self {
		let mut component = Component::new_empty(entry_target.clone());
		let mut other = Component::new_empty(exit_target.clone());
		component.reverse_block = entry_target.clone();
		component.advance_block = exit_target.clone();
		component.incorporate(&mut other);
		component
	}

	/// Reverses all edges and blocks.
	pub fn invert(mut self) -> Self {
		self.blocks.values_mut().for_each(|block| block.invert());
		self
	}

	pub fn append(self, target: BlockTarget, statement: Spanned<Statement<'a>>) -> Self {
		let span = statement.span;
		let block = BasicBlock::new_single(statement);
		let other = Component::new_single(target, block);
		self.join(other, span)
	}

	/// Links the start block to the target block with a jump branch along the advance edge.
	pub fn link_advance(&mut self, start: &BlockTarget, target: &BlockTarget, span: Span) {
		self[start].advance = Spanned::new(Branch::Jump(target.clone()), span);
		self[target].in_advance.push(start.clone());
	}

	/// Links the start block to the target block with a jump branch along the reverse edge.
	pub fn link_reverse(&mut self, start: &BlockTarget, target: &BlockTarget, span: Span) {
		self[start].reverse = Spanned::new(Branch::Jump(target.clone()), span);
		self[target].in_reverse.push(start.clone());
	}

	/// Links the start block with the branch along the advance edge.
	pub fn conditional_advance(&mut self, start: &BlockTarget, branch: ConditionalBranch<'a>, span: Span) {
		self[&branch.target].in_advance.push(start.clone());
		self[&branch.default].in_advance.push(start.clone());
		self[start].advance = Spanned::new(Branch::Conditional(branch), span);
	}

	/// Links the start block with the branch along the reverse edge.
	pub fn conditional_reverse(&mut self, start: &BlockTarget, branch: ConditionalBranch<'a>, span: Span) {
		self[&branch.target].in_reverse.push(start.clone());
		self[&branch.default].in_reverse.push(start.clone());
		self[start].reverse = Spanned::new(Branch::Conditional(branch), span);
	}

	/// Takes the blocks of the other component.
	/// The other components `advance_block` and `reverse_block`
	/// targets will be invalidated.
	pub fn incorporate(&mut self, other: &mut Component<'a>) {
		self.blocks.extend(other.blocks.drain());
	}

	/// Joins the other component onto the endpoint of this component.
	/// Compresses the exit and entry blocks into one if possible.
	pub fn join(mut self, mut other: Self, span: Span) -> Self {
		assert_ne!(self.advance_block, BlockTarget::SENTINEL);
		assert_ne!(other.reverse_block, BlockTarget::SENTINEL);

		self.incorporate(&mut other);
		let advance_block = &self.blocks[&self.advance_block];
		let reverse_block = &self.blocks[&other.reverse_block];
		if advance_block.in_reverse.is_empty() && reverse_block.in_advance.is_empty() {
			let mapping = map! { other.reverse_block.clone() => self.advance_block.clone() };
			let mut reverse_block = self.blocks.remove(&other.reverse_block).unwrap();
			for target in reverse_block.in_reverse {
				self[&target].reverse.replace(&mapping);
			}

			let advance_block = self.blocks.get_mut(&self.advance_block).unwrap();
			advance_block.statements.append(&mut reverse_block.statements);
			advance_block.advance = reverse_block.advance;
			if other.advance_block != other.reverse_block {
				self.advance_block = other.advance_block;
			}
		} else {
			self.link_advance(&self.advance_block.clone(), &other.reverse_block, span);
			self.link_reverse(&other.reverse_block, &self.advance_block.clone(), span);
			self.advance_block = other.advance_block;
		}

		self
	}

	pub fn compress_function(self, function: &crate::node::Function<'a>) -> Function<'a> {
		let mut mapping = HashMap::new();
		for (target, _) in &self.blocks {
			let map_target = BlockTarget(mapping.len());
			mapping.insert(target.clone(), map_target);
		}

		let mut blocks = Vec::new();
		for (_, mut block) in self.blocks {
			block.advance.replace(&mapping);
			block.reverse.replace(&mapping);
			blocks.push(block);
		}

		Function {
			identifier: function.identifier.node.clone(),
			parameters: function.parameters.clone(),
			entry_block: mapping[&self.reverse_block].clone(),
			exit_block: mapping[&self.advance_block].clone(),
			blocks,
		}
	}
}

impl<'a> fmt::Display for Component<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::fmt::Write;
		use crate::utility::IndentWriter;
		let BlockTarget(advance_block) = &self.advance_block;
		let BlockTarget(reverse_block) = &self.reverse_block;

		for (BlockTarget(index), block) in &self.blocks {
			if index == advance_block { write!(f, "+")?; }
			if index == reverse_block { write!(f, "-")?; }
			writeln!(f, "{}:", index)?;

			let mut indent = IndentWriter::wrap(f);
			writeln!(indent, "{}", block)?;
		}
		Ok(())
	}
}

impl<'a> Index<&BlockTarget> for Component<'a> {
	type Output = BasicBlock<'a>;

	fn index(&self, index: &BlockTarget) -> &Self::Output {
		let error = format!("Block target: {}, does not exist in component", index);
		self.blocks.get(index).expect(&error)
	}
}

impl<'a> IndexMut<&BlockTarget> for Component<'a> {
	fn index_mut(&mut self, index: &BlockTarget) -> &mut Self::Output {
		let error = format!("Block target: {}, does not exist in component", index);
		self.blocks.get_mut(index).expect(&error)
	}
}
