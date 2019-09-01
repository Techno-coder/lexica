use crate::node::Accessory;
use crate::source::{Span, Spanned};

use super::{ParserResult, PeekLexer, Token};

pub fn parse_accessories<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                             -> ParserResult<'a, Vec<Accessory<'a>>> {
	let mut accessories = Vec::new();
	loop {
		match lexer.peek() {
			Some(token) if token.node == Token::Accessor =>
				accessories.push(parse_accessory(lexer, end_span)?),
			_ => break Ok(accessories),
		}
	}
}

pub fn parse_accessory<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                           -> ParserResult<'a, Accessory<'a>> {
	expect!(lexer, end_span, Accessor).byte_start;
	let identifier = identifier!(lexer, end_span);
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::ParenthesisOpen =>
			Accessory::FunctionCall(super::parse_function_call(lexer, identifier, end_span)?),
		_ => Accessory::Field(identifier),
	})
}
