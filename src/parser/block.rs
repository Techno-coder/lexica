use crate::node::{DataType, Expression, ExpressionNode, Statement};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub type ExpressionBlock<'a> = (Vec<Spanned<Statement<'a>>>, Spanned<ExpressionNode<'a>>);

pub fn parse_expression_block<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                  -> ParserResult<'a, Spanned<ExpressionBlock<'a>>> {
	let span_start = expect!(lexer, end_span, BlockOpen).byte_start;
	let mut statements = Vec::new();
	let expression = loop {
		match lexer.peek() {
			Some(token) if token.node == Token::BlockClose => {
				let (expression, evaluation_type) = (Expression::Unit, DataType::UNIT_TYPE);
				break Spanned::new(ExpressionNode { expression, evaluation_type }, token.span);
			}
			_ => (),
		}

		let lexer_recovery = lexer.clone();
		let statement = super::parse_statement(lexer, end_span);
		match statement {
			Ok(statement) => statements.push(statement),
			Err(statement_error) => {
				*lexer = lexer_recovery;
				match super::parse_expression_root(lexer, end_span) {
					Ok(expression) => break expression,
					Err(_) => return Err(statement_error),
				}
			}
		}
	};

	let span_end = expect!(lexer, end_span, BlockClose).byte_end;
	Ok(Spanned::new((statements, expression), Span::new(span_start, span_end)))
}

pub fn parse_block<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                       -> ParserResult<'a, Spanned<Vec<Spanned<Statement<'a>>>>> {
	let span_start = expect!(lexer, end_span, BlockOpen).byte_start;
	let mut statements = Vec::new();

	let error = Spanned::new(ParserError::ExpectedToken(Token::BlockClose), end_span);
	while lexer.peek().ok_or(error.clone())?.node != Token::BlockClose {
		statements.push(super::parse_statement(lexer, end_span)?);
	}

	let span_end = expect!(lexer, end_span, BlockClose).byte_end;
	Ok(Spanned::new(statements, Span::new(span_start, span_end)))
}
