use crate::basic::{BlockTarget, Branch, Function};
use crate::interpreter::Direction;
use crate::source::Spanned;

use super::Translator;

type Element = Spanned<super::Element>;

impl<'a, 'b> Translator<'a, 'b> {
	pub fn block_reverse_branch(&mut self, target: &BlockTarget, function: &Function<'a>,
	                            elements: &mut Vec<Element>) {
		let block = &function[target];
		if !function.is_entry() || target != &function.entry_block {
			let branch = self.branch(&block.reverse, Direction::Advance);
			elements.append(&mut self.invert_elements(branch));
		} else {
			let span = block.reverse.span;
			elements.push(instruction!(Reverse, "exit".to_owned(), span));
		}

		let label = format!("{}: pass", self.reverse_mapping(target));
		elements.push(Spanned::new(super::Element::Other(label), block.reverse.span));
	}

	pub fn block_advance_branch(&mut self, target: &BlockTarget, function: &Function<'a>,
	                            elements: &mut Vec<Element>) {
		let block = &function[target];
		let label = format!("{}: pass", self.advance_mapping(target));
		elements.push(Spanned::new(super::Element::Other(label), block.advance.span));

		if !function.is_entry() || target != &function.exit_block {
			let mut branch = self.branch(&block.advance, Direction::Reverse);
			elements.append(&mut branch);
		} else {
			let span = block.advance.span;
			elements.push(instruction!(Advance, "exit".to_owned(), span));
		}
	}

	pub fn block_statements(&mut self, target: &BlockTarget, function: &Function<'a>,
	                        block_elements: &mut Vec<Element>) {
		let block = &function[target];
		for statement in &block.statements {
			let mut elements = Vec::new();
			self.statement(statement, &mut elements);
			if block.direction == Direction::Reverse {
				elements = self.invert_elements(elements);
			}

			block_elements.append(&mut elements);
		}
	}

	pub fn branch(&mut self, branch: &Spanned<Branch<'a>>, target_direction: Direction) -> Vec<Element> {
		let mapping = |target| match target_direction {
			Direction::Advance => self.advance_mapping(target),
			Direction::Reverse => self.reverse_mapping(target),
		};

		let mut elements = Vec::new();
		match &branch.node {
			Branch::Return(expression) => {
				self.drop_expression(expression, &mut elements);
				elements.push(instruction!(Advance, Advance, "return".to_owned(), branch.span));
			}
			Branch::Conditional(conditional) => {
				let (target, default) = (mapping(&conditional.target), mapping(&conditional.default));
				let local = self.promote(&conditional.condition, &mut elements);
				let instruction = format!("branch.i = {} true {}", local, target);
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
				let instruction = format!("jump {}", default);
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
			}
			Branch::Jump(target) => {
				let instruction = format!("jump {}", mapping(target));
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
			}
		}
		elements
	}
}
