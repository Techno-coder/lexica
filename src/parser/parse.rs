use hashbrown::HashMap;

use crate::node::SyntaxUnit;
use crate::source::{ErrorCollate, Span, Spanned, TextMap};

use super::{Lexer, ParserResult, PeekLexer};

pub fn parse(text_map: &TextMap) -> ParserResult<Spanned<SyntaxUnit>> {
	let mut lexer = Lexer::new(text_map);
	let mut errors = ErrorCollate::new();
	let mut functions = HashMap::new();

	while lexer.peek().is_some() {
		let function = super::parse_function(&mut lexer, end_span(text_map.text()));
		match function {
			Ok(function) => {
				let identifier = function.identifier.node.clone();
				functions.insert(identifier, function);
			}
			Err(others) => {
				errors.combine(others);
				discard_state(&mut lexer);
			}
		}
	}

	let span = Span::new(0, text_map.text().len());
	let syntax_unit = Spanned::new(SyntaxUnit { functions }, span);
	errors.collapse(syntax_unit)
}

pub fn end_span(text: &str) -> Span {
	Span::new(text.len(), text.len() + 1)
}

pub fn discard_state(lexer: &mut PeekLexer) {
	while let Some(token) = lexer.peek() {
		match token.structure_separator() {
			false => lexer.next(),
			true => break,
		};
	}
}
