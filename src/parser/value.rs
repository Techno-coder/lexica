use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Expression, ExpressionKey, FunctionContext};
use crate::span::Spanned;

use super::ParserError;

pub fn value(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.peek();
	match &token.node {
		Token::BlockOpen => block(context, lexer),
		other => consume_value(context, lexer),
	}
}

fn consume_value(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.next();
	match token.node {
		Token::Unsigned(unsigned) => Ok(context
			.register(Spanned::new(Expression::Unsigned(unsigned), token.span))),
		Token::Signed(signed) => Ok(context
			.register(Spanned::new(Expression::Signed(signed), token.span))),
		Token::Truth(truth) => Ok(context
			.register(Spanned::new(Expression::Truth(truth), token.span))),
		other => {
			let error = ParserError::ExpectedExpression(other.clone());
			Err(Diagnostic::new(Spanned::new(error, token.span)))
		}
	}
}

fn block(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let mut block = Vec::new();
	let initial_span = super::expect(lexer, Token::BlockOpen)?;
	while lexer.peek().node != Token::BlockClose {
		block.push(super::expression(context, lexer)?);
	}

	let span = initial_span.merge(super::expect(lexer, Token::BlockClose)?);
	Ok(context.register(Spanned::new(Expression::Block(block), span)))
}

