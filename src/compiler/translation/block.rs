use crate::interpreter::Direction;
use crate::source::Spanned;

use super::{Element, FunctionContext};

pub fn polarize(elements: &mut Vec<Spanned<Element>>, direction: Direction) {
	for element in elements {
		if let Element::Instruction(instruction) = &mut element.node {
			instruction.polarization = Some(direction);
		}
	}
}

pub fn polarize_reverse(elements: &mut Vec<Spanned<Element>>) {
	polarize(elements, Direction::Reverse);
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
