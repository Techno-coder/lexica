use crate::node::{Expression, Statement};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_expression_block<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, (Vec<Statement<'a>>, Expression<'a>)> {
	expect!(lexer, end_span, BlockOpen);
	let mut statements = Vec::new();
	let expression = loop {
		let lexer_recovery = lexer.clone();
		let statement = super::parse_statement(lexer, end_span.clone());
		match statement {
			Ok(statement) => statements.push(statement),
			Err(statement_error) => {
				*lexer = lexer_recovery;
				match super::parse_expression_root(lexer, end_span.clone()) {
					Ok(expression) => break expression,
					Err(_) => return Err(statement_error),
				}
			}
		}
	};

	expect!(lexer, end_span, BlockClose);
	Ok((statements, expression))
}

pub fn parse_block<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                       -> ParserResult<'a, Vec<Statement<'a>>> {
	expect!(lexer, end_span, BlockOpen);
	let mut statements = Vec::new();

	let error = Spanned::new(ParserError::ExpectedToken(Token::BlockClose), end_span.clone());
	while lexer.peek().ok_or(error.clone())?.node != Token::BlockClose {
		statements.push(super::parse_statement(lexer, end_span.clone())?);
	}

	expect!(lexer, end_span, BlockClose);
	Ok(statements)
}
