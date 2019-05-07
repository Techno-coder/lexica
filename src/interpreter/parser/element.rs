use super::{Annotation, TranslationInstruction};

/// A singular unit of byte code.
#[derive(Debug, Clone)]
pub enum Element<'a> {
	Annotation(Annotation<'a>),
	Instruction(TranslationInstruction<'a>),
	FunctionLabel(&'a str),
	ReverseLabel(&'a str),
	LocalLabel(&'a str),
	Label(&'a str),
}

impl<'a> Element<'a> {
	/// Returns true if the element takes space in an instruction buffer.
	pub fn advances_counter(&self) -> bool {
		match self {
			Element::Instruction(_) => true,
			_ => false,
		}
	}
}
