use crate::node::{DataType, Identifier, Variable, VariableTarget};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_variable<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<Variable<'a>>> {
	let end_error = Spanned::new(ParserError::ExpectedIdentifier, end_span);
	let mut identifier = lexer.next().ok_or(end_error.clone())?;

	let mut is_mutable = false;
	if let Token::MutableModifier = identifier.node {
		identifier = lexer.next().ok_or(end_error.clone())?;
		is_mutable = true;
	}

	let variable_target: Spanned<VariableTarget> = match identifier.node {
		Token::Identifier(identifier_node) => {
			let variable_target = Identifier(identifier_node).into();
			Spanned::new(variable_target, identifier.span)
		}
		_ => return Err(Spanned::new(ParserError::ExpectedIdentifier, identifier.span).into()),
	};

	let mut span = variable_target.span;
	let data_type = match lexer.peek() {
		Some(separator) => match separator.node {
			Token::VariableSeparator => {
				let _ = lexer.next();
				let identifier = identifier!(lexer, end_span);
				span.byte_end = identifier.span.byte_end;
				DataType::new(identifier.node)
			}
			_ => DataType::default(),
		}
		None => DataType::default(),
	};

	let variable = Variable { target: variable_target.node, data_type, is_mutable };
	Ok(Spanned::new(variable, span))
}

#[cfg(test)]
mod tests {
	use crate::parser::{end_span, Lexer};

	use super::*;

	#[test]
	fn test_identifier() {
		let text = "variable\n";
		let (lexer, end_span) = (&mut Lexer::new(text), end_span(text));

		let target = Identifier("variable").into();
		let variable = Variable { target, data_type: DataType::default(), is_mutable: false };
		assert_eq!(parse_variable(lexer, end_span).unwrap().node, variable);
	}

	#[test]
	fn test_mutable() {
		let text = "~variable\n";
		let (lexer, end_span) = (&mut Lexer::new(text), end_span(text));

		let target = Identifier("variable").into();
		let variable = Variable { target, data_type: DataType::default(), is_mutable: true };
		assert_eq!(parse_variable(lexer, end_span).unwrap().node, variable);
	}
}
