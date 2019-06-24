use crate::node::Function;
use crate::source::Span;

use super::{Lexer, ParserResult};

pub fn parse(text: &str) -> ParserResult<Function> {
	let mut lexer = Lexer::new(text);
	super::parse_function(&mut lexer, end_span(text))
}

pub fn end_span(text: &str) -> Span {
	Span::new(text.len(), text.len() + 1)
}
