use crate::node::Function;
use crate::source::{ErrorCollate, Span, Spanned};

use super::{Lexer, ParserResult, PeekLexer};

pub fn parse(text: &str) -> ParserResult<Vec<Spanned<Function>>> {
	let mut lexer = Lexer::new(text);
	let mut errors = ErrorCollate::new();
	let mut functions = Vec::new();

	while lexer.peek().is_some() {
		let function = super::parse_function(&mut lexer, end_span(text));
		match function {
			Ok(function) => functions.push(function),
			Err(others) => {
				errors.combine(others);
				discard_state(&mut lexer);
			}
		}
	}

	match errors.empty() {
		true => Ok(functions),
		false => Err(errors),
	}
}

pub fn end_span(text: &str) -> Span {
	Span::new(text.len(), text.len() + 1)
}

pub fn discard_state(lexer: &mut PeekLexer) {
	while let Some(token) = lexer.peek() {
		match token.function_separator() {
			false => lexer.next(),
			true => break,
		};
	}
}
