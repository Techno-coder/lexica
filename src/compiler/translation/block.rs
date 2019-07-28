use crate::interpreter::Direction;
use crate::source::Spanned;

use super::{Element, FunctionContext};

/// Composes the polarization with the specified direction.
/// If no polarization exists then the execution direction is composed.
pub fn compose(elements: &mut Vec<Spanned<Element>>, direction: Direction) {
	for element in elements {
		if let Element::Instruction(instruction) = &mut element.node {
			match &mut instruction.polarization {
				Some(polarization) => *polarization = polarization.compose(direction),
				None => instruction.direction = instruction.direction.compose(direction),
			}
		}
	}
}

pub fn compose_reverse(elements: &mut Vec<Spanned<Element>>) {
	compose(elements, Direction::Reverse);
	elements.reverse();
}

/// Drops variables that are yet to be dropped from the removed frame.
pub fn drop_frame(context: &mut FunctionContext, ignored_indexes: &[usize]) -> Vec<Spanned<Element>> {
	let (variable_frame, intermediate_frame) = context.pop_frame();
	let variable_frame = variable_frame.iter().map(|(_, (index, span))| (index, span));
	variable_frame.chain(intermediate_frame.iter())
		.filter(|(index, _)| !ignored_indexes.contains(index))
		.map(|(index, span)| {
			let instruction = format!("drop {}", index);
			instruction!(Advance, instruction, *span)
		}).collect()
}
