use crate::interpreter::Primitive;
use crate::source::Spanned;

use super::{Element, FunctionContext};

#[derive(Debug)]
pub enum Evaluation {
	Unit,
	Local(usize),
	Immediate(Spanned<Primitive>),
}

impl Evaluation {
	pub fn promote(self, elements: &mut Vec<Spanned<Element>>, context: &mut FunctionContext) -> usize {
		match self {
			Evaluation::Unit => panic!("Cannot promote unit evaluation"),
			Evaluation::Local(local) => local,
			Evaluation::Immediate(primitive) => {
				let local = context.register_local(primitive.size());
				let instruction = format!("reset {} {}", local, primitive);
				elements.push(instruction!(Advance, Advance, instruction, primitive.span));
				local
			}
		}
	}
}
