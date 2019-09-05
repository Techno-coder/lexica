use crate::interpreter::{Float, Integer, Primitive};
use crate::node::{Expression, ExpressionNode};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_terminal<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let error = Spanned::new(ParserError::ExpectedExpression, end_span);
	let next_token = lexer.peek().ok_or(error)?;
	match next_token.node {
		Token::Identifier(_) => match_identifier_terminal(lexer, end_span),
		Token::Boolean(boolean) => {
			let (span, _) = (next_token.span, lexer.next());
			let expression = Expression::Primitive(Primitive::Boolean(boolean));
			Ok(Spanned::new(expression.into(), span))
		}
		Token::UnsignedInteger(integer) => {
			let integer = Integer::new_unsigned(integer);
			let (span, _) = (next_token.span, lexer.next());
			let expression = Expression::Primitive(Primitive::Integer(integer));
			Ok(Spanned::new(expression.into(), span))
		}
		Token::SignedInteger(integer) => {
			let integer = Integer::new_signed(integer);
			let (span, _) = (next_token.span, lexer.next());
			let expression = Expression::Primitive(Primitive::Integer(integer));
			Ok(Spanned::new(expression.into(), span))
		}
		Token::Float(float) => {
			let float = Float::Float64(float);
			let (span, _) = (next_token.span, lexer.next());
			let expression = Expression::Primitive(Primitive::Float(float));
			Ok(Spanned::new(expression.into(), span))
		}
		Token::When => {
			let when_conditional = super::parse_when_conditional(lexer, end_span)?;
			let span = when_conditional.span;

			let expression = Expression::WhenConditional(when_conditional);
			Ok(Spanned::new(expression.into(), span))
		}
		Token::BlockOpen => {
			let expression_block = super::parse_expression_block(lexer, end_span)?;
			let span = expression_block.span;

			let expression = Expression::ExpressionBlock(expression_block);
			Ok(Spanned::new(expression.into(), span))
		}
		_ => Err(Spanned::new(ParserError::ExpectedExpression, next_token.span).into()),
	}
}

pub fn match_identifier_terminal<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                     -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let identifier = identifier!(lexer, end_span);
	// TODO: Parse AccessorCall
	let expression_variable = Expression::Variable(identifier.node.clone().into());
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::ParenthesisOpen => {
			let function_call = super::parse_function_call(lexer, identifier, end_span)?;
			let (function_call, span) = (function_call.node, function_call.span);
			let expression = Expression::FunctionCall(Spanned::new(function_call, span));
			Spanned::new(expression.into(), span)
		}
		_ => Spanned::new(expression_variable.into(), identifier.span),
	})
}
