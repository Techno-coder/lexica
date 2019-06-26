use std::collections::HashMap;
use std::ops::DerefMut;

use crate::compiler::{Element, Instruction};
use crate::interpreter::{Direction, Size};
use crate::node::*;
use crate::source::{Span, Spanned};

#[derive(Debug, Default)]
pub struct Translator<'a> {
	allocation_sizes: Vec<Size>,
	identifier_table: HashMap<Identifier<'a>, usize>,
	identifier_table_spans: HashMap<Identifier<'a>, Span>,
	expression_stack: Vec<usize>,
	label_index: usize,
}

impl<'a> NodeVisitor<'a> for Translator<'a> {
	type Result = Vec<Spanned<Element>>;

	fn binary_operation(&mut self, operation: &mut Spanned<&mut BinaryOperation<'a>>) -> Self::Result {
		let mut elements = operation.left.accept(self);
		let left = self.expression_stack.pop().unwrap();
		elements.append(&mut operation.right.accept(self));
		let right = self.expression_stack.pop().unwrap();

		let local_index = self.allocation_sizes.len();
		self.expression_stack.push(local_index);
		match *operation.operator {
			BinaryOperator::Equal => {
				self.allocation_sizes.push(Size::Boolean);
				let instruction = format!("compare = {} {} {}", left, right, local_index);
				elements.push(instruction!(Advance, instruction, operation.span));
			}
			BinaryOperator::Add => {
				self.allocation_sizes.push(self.allocation_sizes[left].clone());
				let instruction = format!("clone {} {}", local_index, left);
				elements.push(instruction!(Advance, Advance, instruction, operation.span));
				let instruction = format!("add {} {}", local_index, right);
				elements.push(instruction!(Advance, instruction, operation.operator.span));
			}
			BinaryOperator::Minus => {
				self.allocation_sizes.push(self.allocation_sizes[left].clone());
				let instruction = format!("clone {} {}", local_index, left);
				elements.push(instruction!(Advance, Advance, instruction, operation.span));
				let instruction = format!("minus {} {}", local_index, right);
				elements.push(instruction!(Advance, instruction, operation.operator.span));
			}
			_ => unimplemented!(), // TODO
		};
		elements
	}

	fn binding(&mut self, binding: &mut Spanned<Binding<'a>>) -> Self::Result {
		let elements = binding.expression.accept(self);
		let identifier = binding.variable.identifier.clone();
		let local_index = self.expression_stack.pop().unwrap();
		self.identifier_table.insert(identifier.clone(), local_index);
		self.identifier_table_spans.insert(identifier, binding.span);
		elements
	}

	fn conditional_loop(&mut self, conditional_loop: &mut Spanned<ConditionalLoop<'a>>) -> Self::Result {
		let (start_label, end_label) = (self.label_index, self.label_index + 1);
		self.label_index += 2;

		let node_span = conditional_loop.span;
		let mut elements = vec![Spanned::new(Element::Other(format!("{}:", start_label)), node_span)];
		elements.push(instruction!(Advance, "pass".to_owned(), node_span));
		elements.push(instruction!(Advance, Reverse, format!("jump {}", end_label), node_span));

		let end_condition = &mut conditional_loop.end_condition;
		elements.append(&mut end_condition.accept(self).into_iter()
			.map(|element| Spanned::new(match element.node {
				Element::Instruction(mut instruction) => {
					instruction.polarization = Some(Direction::Advance);
					Element::Instruction(instruction)
				}
				other => other,
			}, element.span)).collect());
		let expression_index = self.expression_stack.pop().unwrap();
		let instruction = format!("branch.i = {} true {}", expression_index, end_label);
		elements.push(instruction!(Advance, Advance, instruction, end_condition.span));

		for statement in &mut conditional_loop.statements {
			elements.append(&mut statement.accept(self));
		}

		let start_condition = conditional_loop.start_condition.as_mut().unwrap();
		let mut condition_elements = start_condition.accept(self).into_iter()
			.map(|element| Spanned::new(match element.node {
				Element::Instruction(mut instruction) => {
					instruction.polarization = Some(Direction::Reverse);
					Element::Instruction(instruction)
				}
				other => other,
			}, element.span)).rev().collect();
		let expression_index = self.expression_stack.pop().unwrap();
		let instruction = format!("branch.i = {} true {}", expression_index, start_label);
		elements.push(instruction!(Advance, Reverse, instruction, start_condition.span));
		elements.append(&mut condition_elements);

		elements.push(instruction!(Advance, Advance, format!("jump {}", start_label), node_span));
		elements.push(Spanned::new(Element::Other(format!("{}:", end_label)), node_span));
		elements
	}

	fn explicit_drop(&mut self, explicit_drop: &mut Spanned<ExplicitDrop<'a>>) -> Self::Result {
		let mut elements: Vec<_> = explicit_drop.expression.accept(self)
			.into_iter().map(|element| Spanned::new(match element.node {
			// TODO: Identify correct method of polarization reversal
			Element::Instruction(mut instruction) => {
				instruction.polarization = Some(Direction::Reverse);
				Element::Instruction(instruction)
			}
			other => other,
		}, element.span)).rev().collect();

		let local_index = self.identifier_table.remove(&explicit_drop.identifier).unwrap();
		self.identifier_table.remove(&explicit_drop.identifier);
		let expression_index = self.expression_stack.pop().unwrap();
		let instruction = format!("clone {} {}", local_index, expression_index);
		elements.insert(0, instruction!(Advance, Reverse, instruction, explicit_drop.span));
		elements
	}

	fn expression(&mut self, expression: &mut Spanned<Expression<'a>>) -> Self::Result {
		match &mut expression.node {
			Expression::Variable(variable) => {
				let variable_index = self.identifier_table[variable];
				self.expression_stack.push(variable_index);
				vec![]
			}
			Expression::LiteralInteger(integer) => {
				let local_index = self.allocation_sizes.len();
				self.allocation_sizes.push(Size::Signed64);
				self.expression_stack.push(local_index);
				let instruction = format!("reset {} {}", local_index, integer);
				vec![instruction!(Advance, Advance, instruction, expression.span)]
			}
			Expression::BinaryOperation(operation) => {
				let operation = Box::deref_mut(operation);
				Spanned::new(operation, expression.span).accept(self)
			}
		}
	}

	fn function(&mut self, function: &mut Spanned<Function<'a>>) -> Self::Result {
		for parameter in &function.parameters {
			let local_index = self.allocation_sizes.len();
			// TODO: Parse data types
			self.allocation_sizes.push(Size::Unsigned64);
			self.identifier_table.insert(parameter.identifier.clone(), local_index);
			self.identifier_table_spans.insert(parameter.identifier.clone(), parameter.span);
		}

		let mut central_elements: Vec<_> = function.statements.iter_mut()
			.flat_map(|statement| statement.accept(self)).collect();
		central_elements.append(&mut function.return_value.accept(self));

		let mut elements = Vec::new();
		for allocation_size in &self.allocation_sizes {
			let annotation = match allocation_size {
				Size::Boolean => format!("@local {} false", allocation_size),
				_ => format!("@local {} 0", allocation_size),
			};
			elements.push(Spanned::new(Element::Other(annotation), function.span));
		}

		let span = function.identifier.span;
		elements.push(Spanned::new(Element::Other(format!("~{} {{", function.identifier)), span));
		elements.push(instruction!(Reverse, Reverse, "return".to_owned(), function.span));

		for (parameter_index, parameter) in function.parameters.iter().enumerate() {
			let instruction = format!("restore {}", parameter_index);
			elements.push(instruction!(Advance, instruction, parameter.span));
		}
		elements.append(&mut central_elements);

		let return_index = self.expression_stack.pop().unwrap();
		for (identifier, identifier_index) in &self.identifier_table {
			if *identifier_index == return_index {
				continue;
			}

			let span = self.identifier_table_spans[identifier];
			let instruction = format!("drop {}", identifier_index);
			elements.push(instruction!(Advance, instruction, span));
		}

		let span = function.return_value.span;
		elements.push(instruction!(Advance, format!("drop {}", return_index), span));
		elements.push(instruction!(Advance, Advance, "return".to_owned(), span));
		elements.push(Spanned::new(Element::Other("}".to_owned()), function.span));
		elements
	}

	fn mutation(&mut self, mutation: &mut Spanned<Mutation<'a>>) -> Self::Result {
		let span = mutation.span;
		match &mut mutation.node {
			Mutation::Swap(left, right) => {
				let left = self.identifier_table[&left];
				let right = self.identifier_table[&right];
				vec![instruction!(Advance, format!("swap {} {}", left, right), span)]
			}
			Mutation::AddAssign(identifier, expression) => {
				let expression_elements = expression.accept(self);
				let mut elements = expression_elements.clone();

				let temporary = self.expression_stack.pop().unwrap();
				let local_index = self.identifier_table[identifier];
				elements.push(instruction!(Advance, format!("add {} {}", local_index, temporary), span));

				elements.append(&mut expression_elements.into_iter()
					.map(|element| Spanned::new(match element.node {
						// TODO: Identify correct method of polarization reversal
						Element::Instruction(mut instruction) => {
							instruction.polarization = Some(Direction::Reverse);
							Element::Instruction(instruction)
						}
						other => other,
					}, element.span)).collect());
				elements
			}
			Mutation::MultiplyAssign(_, _) => unimplemented!(),
		}
	}

	fn statement(&mut self, statement: &mut Spanned<Statement<'a>>) -> Self::Result {
		match &mut statement.node {
			Statement::Binding(binding) => binding.accept(self),
			Statement::Mutation(mutation) => mutation.accept(self),
			Statement::ExplicitDrop(explicit_drop) => explicit_drop.accept(self),
			Statement::ConditionalLoop(conditional_loop) => conditional_loop.accept(self),
		}
	}
}
