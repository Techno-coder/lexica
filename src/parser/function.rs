use crate::node::{DataType, Function, Variable};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_function<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<Function<'a>>> {
	let span_start = expect!(lexer, end_span, Function).byte_start;
	let identifier = identifier!(lexer, end_span);
	let parameters = parse_parameter_list(lexer, end_span)?;

	let return_type = match lexer.peek() {
		Some(token) if token.node == Token::ReturnSeparator => {
			let _ = lexer.next();
			let return_type = identifier!(lexer, end_span);
			Spanned::new(DataType::new(return_type.node), return_type.span)
		}
		_ => Spanned::new(DataType::UNIT_TYPE, identifier.span),
	};

	let expression_block = super::parse_expression_block(lexer, end_span)?;
	let span = Span::new(span_start, expression_block.span.byte_end);
	let function = Function { identifier, parameters, expression_block, return_type };
	Ok(Spanned::new(function, span))
}

pub fn parse_parameter_list<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                -> ParserResult<'a, Vec<Spanned<Variable<'a>>>> {
	expect!(lexer, end_span, ParenthesisOpen);
	let mut parameters = Vec::new();
	let mut separator_taken = true;
	loop {
		let brace_error = ParserError::ExpectedToken(Token::ParenthesisClose);
		let spanned_brace_error = Spanned::new(brace_error, end_span);
		let token = lexer.peek().ok_or(spanned_brace_error.clone())?;

		if let Token::ParenthesisClose = token.node {
			let _ = lexer.next();
			break;
		}

		match separator_taken {
			true => separator_taken = false,
			false => {
				let error = ParserError::ExpectedToken(Token::ListSeparator);
				return Err(Spanned::new(error, token.span).into());
			}
		}

		let parameter = super::parse_variable(lexer, end_span)?;
		parameters.push(parameter);

		if let Token::ListSeparator = lexer.peek().ok_or(spanned_brace_error)?.node {
			separator_taken = true;
		}
	}
	Ok(parameters)
}
