use crate::node::{DataType, Identifier, Variable};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_variable<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Variable<'a>> {
	let end_error = Spanned::new(ParserError::ExpectedIdentifier, end_span.clone());
	let mut identifier = lexer.next().ok_or(end_error.clone())?;

	let mut is_mutable = false;
	if let Token::MutableModifier = identifier.node {
		identifier = lexer.next().ok_or(end_error.clone())?;
		is_mutable = true;
	}

	let identifier = match identifier.node {
		Token::Identifier(identifier_node) => {
			let identifier_node = Identifier(identifier_node);
			Spanned::new(identifier_node, identifier.span)
		}
		_ => return Err(Spanned::new(ParserError::ExpectedIdentifier, identifier.span).into()),
	};

	let data_type = match lexer.peek() {
		Some(separator) => match separator.node {
			Token::VariableSeparator => {
				let _ = lexer.next();
				Some(DataType(identifier!(lexer, end_span).node))
			}
			_ => None,
		}
		None => None,
	};

	Ok(Variable { identifier: identifier.node, data_type, is_mutable })
}

#[cfg(test)]
mod tests {
	use crate::parser::{end_span, Lexer};

	use super::*;

	#[test]
	fn test_identifier() {
		let text = "variable\n";
		let lexer = Lexer::new(text);
		let (lexer, end_span) = (&mut Lexer::new(text), end_span(text));

		let identifier = Identifier("variable");
		let variable = Variable { identifier, data_type: None, is_mutable: false };
		assert_eq!(parse_variable(lexer, end_span).unwrap(), variable);
	}

	#[test]
	fn test_mutable() {
		let text = "~variable\n";
		let lexer = Lexer::new(text);
		let (lexer, end_span) = (&mut Lexer::new(text), end_span(text));

		let identifier = Identifier("variable");
		let variable = Variable { identifier, data_type: None, is_mutable: true };
		assert_eq!(parse_variable(lexer, end_span).unwrap(), variable);
	}
}
