use hashbrown::HashMap;

use crate::basic::*;
use crate::interpreter::{Direction, Size};
use crate::intrinsics::IntrinsicStore;
use crate::node::{BinaryOperator, Identifier, Variable, VariableTarget};
use crate::source::{Span, Spanned};

type Element = Spanned<super::Element>;

#[derive(Debug)]
pub struct Translator<'a, 'b> {
	locals: Vec<Size>,
	bindings: HashMap<VariableTarget<'a>, usize>,
	/// Stores the index of the last element encountered when reversing a block.
	reverse_mapping: HashMap<BlockTarget, usize>,
	/// Stores the index of the last element encountered when advancing a block.
	advance_mapping: HashMap<BlockTarget, usize>,
	intrinsics: &'b IntrinsicStore,
	elements: Vec<Element>,
}

impl<'a, 'b> Translator<'a, 'b> {
	pub fn new(intrinsics: &'b IntrinsicStore) -> Self {
		Translator {
			locals: Vec::new(),
			bindings: HashMap::new(),
			reverse_mapping: HashMap::new(),
			advance_mapping: HashMap::new(),
			intrinsics,
			elements: Vec::new(),
		}
	}

	pub fn translate(&mut self, functions: Vec<Spanned<Function<'a>>>) -> Vec<Element> {
		functions.into_iter().for_each(|function| self.translate_function(function));
		std::mem::replace(&mut self.elements, Vec::new())
	}

	pub fn translate_function(&mut self, function: Spanned<Function<'a>>) {
		self.locals.clear();
		self.bindings.clear();
		self.generate_labels(&function);

		let mut block_elements = Vec::new();
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

		self.translate_entry(&function, &mut block_elements);
		for (index, _) in function.blocks.iter().enumerate() {
			let target = BlockTarget(index);
			if target != function.entry_block && target != function.exit_block {
				self.translate_block(&target, &function, &mut block_elements);
			}
		}

		if function.exit_block != function.entry_block {
			self.translate_block(&function.exit_block, &function, &mut block_elements);
		}

		for local in &self.locals {
			let annotation = super::Element::Other(format!("@local {}", local));
			self.elements.push(Spanned::new(annotation, function.span));
		}

		let header = super::Element::Other(format!("~{} {{", function.identifier));
		self.elements.push(Spanned::new(header, function.span));
		self.elements.append(&mut block_elements);
		self.elements.push(Spanned::new(super::Element::Other("}".to_owned()), function.span));
	}

	pub fn generate_labels(&mut self, function: &Function<'a>) {
		let mut next_label = 0;
		self.reverse_mapping.clear();
		self.advance_mapping.clear();
		for (index, _) in function.blocks.iter().enumerate() {
			let block_target = BlockTarget(index);
			self.reverse_mapping.insert(block_target.clone(), next_label);
			self.advance_mapping.insert(block_target, next_label + 1);
			next_label += 2;
		}
	}

	pub fn register_variable(&mut self, variable: &Variable<'a>) -> usize {
		let size = Size::parse(variable.data_type.resolved().unwrap())
			.expect("Invalid size type for binding");
		let index = self.register_local(size);
		self.bindings.insert(variable.target.clone(), index);
		index
	}

	pub fn register_local(&mut self, size: Size) -> usize {
		self.locals.push(size);
		self.locals.len() - 1
	}

	pub fn invert_elements(&self, mut elements: Vec<Element>) -> Vec<Element> {
		elements.iter_mut().for_each(|element| element.invert());
		elements.reverse();
		elements
	}

	pub fn translate_entry(&mut self, function: &Function<'a>, block_elements: &mut Vec<Element>) {
		let target = &function.entry_block;
		self.block_reverse_branch(target, function, block_elements);

		for parameter in &function.parameters {
			let instruction = format!("restore {}", self.bindings[&parameter.target]);
			let instruction = instruction!(Advance, instruction, parameter.span);
			block_elements.push(instruction);
		}

		self.block_statements(target, function, block_elements);
		self.block_advance_branch(target, function, block_elements);
	}

	pub fn translate_block(&mut self, target: &BlockTarget, function: &Function<'a>,
	                       block_elements: &mut Vec<Element>) {
		self.block_reverse_branch(target, function, block_elements);
		self.block_statements(target, function, block_elements);
		self.block_advance_branch(target, function, block_elements);
	}

	pub fn block_reverse_branch(&mut self, target: &BlockTarget, function: &Function<'a>,
	                            block_elements: &mut Vec<Element>) {
		let block = &function[target];
		if !function.is_entry() || target != &function.entry_block {
			let branch = self.branch(&block.reverse, Direction::Advance);
			block_elements.append(&mut self.invert_elements(branch));
		} else {
			let span = block.reverse.span;
			block_elements.push(instruction!(Reverse, "exit".to_owned(), span));
		}

		let label = format!("{}:", self.reverse_mapping[target]);
		block_elements.push(Spanned::new(super::Element::Other(label), block.reverse.span));
	}

	pub fn block_advance_branch(&mut self, target: &BlockTarget, function: &Function<'a>,
	                            block_elements: &mut Vec<Element>) {
		let block = &function[target];
		let label = format!("{}:", self.advance_mapping[target]);
		block_elements.push(Spanned::new(super::Element::Other(label), block.advance.span));

		if !function.is_entry() || target != &function.exit_block {
			let mut branch = self.branch(&block.advance, Direction::Reverse);
			block_elements.append(&mut branch);
		} else {
			let span = block.advance.span;
			block_elements.push(instruction!(Advance, "exit".to_owned(), span));
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
		let mapping = match target_direction {
			Direction::Advance => &self.advance_mapping,
			Direction::Reverse => &self.reverse_mapping,
		};

		let mut elements = Vec::new();
		match &branch.node {
			Branch::Return(expression) => {
				self.drop_expression(expression, &mut elements);
				elements.push(instruction!(Advance, Advance, "return".to_owned(), branch.span));
			}
			Branch::Conditional(conditional) => {
				let (target, default) = (mapping[&conditional.target], mapping[&conditional.default]);
				let local = self.promote(&conditional.condition, &mut elements);
				let instruction = format!("branch.i = {} true {}", local, target);
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
				let instruction = format!("jump {}", default);
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
			}
			Branch::Jump(target) => {
				let instruction = format!("jump {}", mapping[target]);
				elements.push(instruction!(Advance, Advance, instruction, branch.span));
			}
		}
		elements
	}

	pub fn statement(&mut self, statement: &Statement<'a>, elements: &mut Vec<Element>) {
		match statement {
			Statement::Binding(binding) => self.binding(binding, elements),
			Statement::Mutation(mutation) => self.mutation(mutation, elements),
			Statement::Assignment(assignment) => self.assignment(assignment, elements),
			Statement::FunctionCall(function_call) => self.function_call(function_call, elements),
			Statement::ImplicitDrop(implicit_drop) => self.implicit_drop(implicit_drop, elements),
		}
	}

	pub fn register_binding(&mut self, binding: &Spanned<Binding<'a>>) {
		match &binding.value {
			Value::Uninitialized(_) => return,
			Value::Expression(expression) if expression.is_unit() => return,
			_ => self.register_variable(&binding.variable),
		};
	}

	pub fn binding(&mut self, binding: &Spanned<Binding<'a>>, elements: &mut Vec<Element>) {
		let span = binding.span;
		match &binding.value {
			Value::Uninitialized(_) => return,
			Value::Expression(expression) => match expression.is_unit() {
				false => {
					let local = self.bindings[&binding.variable.target];
					self.assign_expression(local, expression, elements)
				}
				true => return,
			},
			Value::FunctionCall(function_call) => {
				let local = self.bindings[&binding.variable.target];
				self.function_call(function_call, elements);
				let instruction = format!("restore {}", local);
				elements.push(instruction!(Advance, instruction, function_call.span));
			}
			Value::BinaryOperation(binary_operation) => {
				let local = self.bindings[&binding.variable.target];
				let operation = match binary_operation.operator.node {
					BinaryOperator::Equal => {
						let left = self.promote(&binary_operation.left, elements);
						let right = self.promote(&binary_operation.right, elements);
						let instruction = format!("compare = {} {} {}", left, right, local);
						elements.push(instruction!(Advance, Advance, instruction, span));
						return;
					}
					BinaryOperator::Add => "add",
					BinaryOperator::Minus => "minus",
					BinaryOperator::Multiply => "multiply",
				};

				self.assign_expression(local, &binary_operation.left, elements);
				self.mutate(local, &binary_operation.right, operation, elements);
			}
		}
	}

	pub fn promote(&mut self, expression: &Spanned<Expression<'a>>, elements: &mut Vec<Element>) -> usize {
		match &expression.node {
			Expression::Unit => panic!("Unit type cannot be promoted"),
			Expression::Variable(variable) => self.bindings[&variable.target],
			Expression::Primitive(_) => {
				let size = Size::parse(expression.data_type().resolved().unwrap())
					.expect("Invalid size type for expression");
				let local = self.register_local(size);
				self.assign_expression(local, expression, elements);
				local
			}
		}
	}

	pub fn mutation(&mut self, mutation: &Spanned<Mutation<'a>>, elements: &mut Vec<Element>) {
		let (target, expression, operation) = match &mutation.node {
			Mutation::Swap(left, right) => {
				let instruction = format!("swap {} {}", self.bindings[&left], self.bindings[&right]);
				return elements.push(instruction!(Advance, instruction, mutation.span));
			}
			Mutation::AddAssign(target, expression) => (target, expression, "add"),
			Mutation::MinusAssign(target, expression) => (target, expression, "minus"),
			Mutation::MultiplyAssign(target, expression) => (target, expression, "multiply"),
		};

		let local = self.bindings[target];
		self.mutate(local, expression, operation, elements);
	}

	pub fn mutate(&mut self, local: usize, expression: &Spanned<Expression<'a>>,
	              operation: &str, elements: &mut Vec<Element>) {
		elements.push(instruction!(Advance, match &expression.node {
			Expression::Unit => return,
			Expression::Variable(variable) => {
				let other = self.bindings[&variable.target];
				format!("{} {} {}", operation, local, other)
			},
			Expression::Primitive(primitive) =>
				format!("{}.i {} {}", operation, local, primitive),
		}, expression.span));
	}

	pub fn function_call(&mut self, function_call: &Spanned<FunctionCall<'a>>, elements: &mut Vec<Element>) {
		let Identifier(function) = function_call.function.node;
		function_call.arguments.iter().for_each(|argument| self.drop_expression(argument, elements));

		let instruction = format!("call {}", function);
		elements.push(match self.intrinsics.get(function) {
			Some(_) => instruction!(Advance, Advance, instruction, function_call.function.span),
			None => instruction!(Advance, instruction, function_call.function.span),
		})
	}

	pub fn drop_expression(&self, expression: &Spanned<Expression<'a>>, elements: &mut Vec<Element>) {
		let span = expression.span;
		match &expression.node {
			Expression::Unit => (),
			Expression::Variable(variable) => self.drop_target(&variable.target, span, elements),
			Expression::Primitive(primitive) => {
				let instruction = format!("drop.i {} {}", primitive.size(), primitive);
				elements.push(instruction!(Advance, instruction, span));
			}
		}
	}

	pub fn drop_target(&self, target: &VariableTarget<'a>, span: Span, elements: &mut Vec<Element>) {
		let instruction = format!("drop {}", self.bindings[&target]);
		elements.push(instruction!(Advance, instruction, span));
	}

	pub fn assignment(&mut self, assignment: &Spanned<Assignment<'a>>, elements: &mut Vec<Element>) {
		let local = self.bindings[&assignment.target];
		self.assign_expression(local, &assignment.expression, elements);
	}

	pub fn assign_expression(&mut self, local: usize, expression: &Spanned<Expression<'a>>,
	                         elements: &mut Vec<Element>) {
		let span = expression.span;
		match &expression.node {
			Expression::Unit => (),
			Expression::Variable(variable) => {
				let other = self.bindings[&variable.target];
				let instruction = format!("clone {} {}", local, other);
				let instruction = instruction!(Advance, Advance, instruction, span);
				elements.push(instruction);
			}
			Expression::Primitive(primitive) => {
				let instruction = format!("reset {} {}", local, primitive);
				let instruction = instruction!(Advance, Advance, instruction, span);
				elements.push(instruction);
			}
		}
	}

	pub fn implicit_drop(&mut self, implicit_drop: &Spanned<ImplicitDrop<'a>>, elements: &mut Vec<Element>) {
		self.drop_target(&implicit_drop.target, implicit_drop.span, elements);
	}
}
