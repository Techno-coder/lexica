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
	let mut elements = Vec::new();
	let frame = context.pop_frame();
	for (_, (identifier_index, span)) in frame {
		match ignored_indexes.contains(&identifier_index) {
			true => continue,
			false => {
				let instruction = format!("drop {}", identifier_index);
				elements.push(instruction!(Advance, instruction, span));
			}
		}
	}
	elements
}
