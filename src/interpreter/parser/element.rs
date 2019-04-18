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
	ReversalHint,
}

impl<'a> Element<'a> {
	/// Returns true if the element takes space in an instruction buffer.
	pub fn advances_counter(&self) -> bool {
		match self {
			Element::Instruction(_) => true,
			Element::ReversalHint => true,
			_ => false,
		}
	}
}
