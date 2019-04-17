use super::{Annotation, TranslationInstruction};

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