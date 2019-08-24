use crate::basic::{BlockTarget, Function, Statement};
use crate::source::Spanned;

use super::Translator;

type Element = Spanned<super::Element>;

impl<'a, 'b> Translator<'a, 'b> {
	pub fn register_function_bindings(&mut self, function: &Function<'a>) {
		for parameter in &function.parameters {
			self.register_variable(parameter);
		}

		for block in &function.blocks {
			for statement in &block.statements {
				if let Statement::Binding(binding) = &statement.node {
					self.register_binding(binding);
				}
			}
		}
	}

	pub fn translate_function_blocks(&mut self, function: &Function<'a>, elements: &mut Vec<Element>) {
		self.translate_entry(&function, elements);
		for (index, _) in function.blocks.iter().enumerate() {
			let target = BlockTarget(index);
			if target != function.entry_block && target != function.exit_block {
				self.translate_block(&target, &function, elements);
			}
		}

		if function.exit_block != function.entry_block {
			self.translate_block(&function.exit_block, &function, elements);
		}
	}

	pub fn translate_entry(&mut self, function: &Function<'a>, elements: &mut Vec<Element>) {
		let target = &function.entry_block;
		self.block_reverse_branch(target, function, elements);

		for parameter in function.parameters.iter().rev() {
			let instruction = format!("restore {}", self.binding_local(&parameter.target));
			let instruction = instruction!(Advance, instruction, parameter.span);
			elements.push(instruction);
		}

		self.block_statements(target, function, elements);
		self.block_advance_branch(target, function, elements);
	}

	pub fn translate_block(&mut self, target: &BlockTarget, function: &Function<'a>,
	                       elements: &mut Vec<Element>) {
		self.block_reverse_branch(target, function, elements);
		self.block_statements(target, function, elements);
		self.block_advance_branch(target, function, elements);
	}
}
