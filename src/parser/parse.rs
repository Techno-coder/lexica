use hashbrown::HashMap;

use crate::node::SyntaxUnit;
use crate::source::{ErrorCollate, Span, Spanned, TextMap};

use super::{Lexer, ParserError, ParserResult, PeekLexer, Token};

pub fn parse(text_map: &TextMap) -> ParserResult<Spanned<SyntaxUnit>> {
	let mut lexer = Lexer::new(text_map);
	let mut errors = ErrorCollate::new();

	let mut functions = HashMap::new();
	let mut structures = HashMap::new();

	let end_span = end_span(text_map.text());
	while let Some(token) = lexer.peek() {
		let result = match token.node {
			Token::Function => super::parse_function(&mut lexer, end_span)
				.map(|function| {
					let identifier = function.identifier.node.clone();
					functions.insert(identifier, function);
				}),
			Token::Data => super::parse_structure(&mut lexer, end_span)
				.map(|structure| {
					let identifier = structure.identifier.node.clone();
					structures.insert(identifier, structure);
				}),
			_ => Err(Spanned::new(ParserError::ExpectedStructure, token.span).into()),
		};

		if let Err(others) = result {
			errors.combine(others);
			discard_state(&mut lexer);
		}
	}

	let span = Span::new(0, text_map.text().len());
	let syntax_unit = Spanned::new(SyntaxUnit { structures, functions }, span);
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
