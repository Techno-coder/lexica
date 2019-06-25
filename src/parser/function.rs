use crate::node::{Function, Variable};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_function<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Function<'a>> {
	expect!(lexer, end_span, Function);
	let identifier = identifier!(lexer, end_span).node;
	let parameters = parse_parameter_list(lexer, end_span.clone())?;

	// TODO: Consider parsing return type
	let (statements, return_value) = super::parse_expression_block(lexer, end_span.clone())?;
	Ok(Function { identifier, parameters, statements, return_value })
}

pub fn parse_parameter_list<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                -> ParserResult<'a, Vec<Variable<'a>>> {
	expect!(lexer, end_span, ParenthesisOpen);
	let mut parameters = Vec::new();
	let mut separator_taken = true;
	loop {
		let brace_error = ParserError::ExpectedToken(Token::ParenthesisClose);
		let spanned_brace_error = Spanned::new(brace_error, end_span.clone());
		let token = lexer.peek().ok_or(spanned_brace_error.clone())?;

		if let Token::ParenthesisClose = token.node {
			let _ = lexer.next();
			break;
		}

		match separator_taken {
			true => separator_taken = false,
			false => {
				let error = ParserError::ExpectedToken(Token::ListSeparator);
				return Err(Spanned::new(error, token.span.clone()).into());
			}
		}

		let parameter = super::parse_variable(lexer, end_span.clone())?;
		parameters.push(parameter);

		if let Token::ListSeparator = lexer.peek().ok_or(spanned_brace_error)?.node {
			separator_taken = true;
		}
	}
	Ok(parameters)
}
