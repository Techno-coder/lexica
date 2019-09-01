use hashbrown::HashMap;

use crate::basic::*;
use crate::interpreter::Size;
use crate::intrinsics::IntrinsicStore;
use crate::node::{Identifier, Variable, VariableTarget, StructureMap};
use crate::source::Spanned;

type Element = Spanned<super::Element>;

#[derive(Debug)]
pub struct Translator<'a, 'b> {
	locals: Vec<Size>,
	bindings: HashMap<VariableTarget<'a>, usize>,
	/// Stores the index of the last element encountered when reversing a block.
	reverse_mapping: HashMap<BlockTarget, usize>,
	/// Stores the index of the last element encountered when advancing a block.
	advance_mapping: HashMap<BlockTarget, usize>,

	structures: StructureMap<'a>,
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
			structures: HashMap::new(),
			intrinsics,
			elements: Vec::new(),
		}
	}

	pub fn translate(&mut self, unit: BasicUnit<'a>) -> Vec<Element> {
		self.structures = unit.structures.into_iter()
			.map(|structure| (structure.identifier.node.clone(), structure)).collect();
		unit.functions.into_iter().for_each(|function| self.translate_function(function));
		std::mem::replace(&mut self.elements, Vec::new())
	}

	pub fn translate_function(&mut self, function: Spanned<Function<'a>>) {
		self.locals.clear();
		self.bindings.clear();
		self.generate_labels(&function);

		let mut block_elements = Vec::new();
		self.register_function_bindings(&function);
		self.translate_function_blocks(&function, &mut block_elements);

		for local in &self.locals {
			let annotation = super::Element::Other(format!("@local {}", local));
			self.elements.push(Spanned::new(annotation, function.span));
		}

		let header = super::Element::Other(format!("~{} {{", function.identifier));
		self.elements.push(Spanned::new(header, function.span));
		self.elements.append(&mut block_elements);

		let element = super::Element::Other("}".to_owned());
		self.elements.push(Spanned::new(element, function.span));
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

	pub fn binding_local(&self, target: &VariableTarget<'a>) -> usize {
		let error = format!("Binding for target: {}, does not exist", target);
		self.bindings.get(target).expect(&error).clone()
	}

	pub fn reverse_mapping(&self, target: &BlockTarget) -> usize {
		let error = format!("Reverse mapping for target: {}, does not exist", target);
		self.reverse_mapping.get(target).expect(&error).clone()
	}

	pub fn advance_mapping(&self, target: &BlockTarget) -> usize {
		let error = format!("Advance mapping for target: {}, does not exist", target);
		self.advance_mapping.get(target).expect(&error).clone()
	}

	pub fn is_intrinsic(&self, identifier: &Identifier<'a>) -> bool {
		let Identifier(function) = identifier;
		self.intrinsics.get(function).is_some()
	}

	pub fn invert_elements(&self, mut elements: Vec<Element>) -> Vec<Element> {
		elements.iter_mut().for_each(|element| element.invert());
		elements.reverse();
		elements
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
}
