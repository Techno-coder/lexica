use crate::interpreter::Direction;
use crate::source::Spanned;

use super::Element;

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
