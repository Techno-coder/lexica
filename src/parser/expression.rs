use crate::node::{BinaryOperation, BinaryOperator, DataType, Expression, ExpressionNode,
	FunctionCall, Identifier, Accessor};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_expression_root<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                 -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let expression = parse_expression(lexer, end_span, 0)?;
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::Accessor => {
			let evaluation_type = DataType::default();
			let accessories = super::parse_accessories(lexer, end_span)?;
			let byte_end = accessories.last().unwrap().span().byte_end;

			let span = Span::new(expression.span.byte_start, byte_end);
			let accessor = Accessor { expression, accessories, evaluation_type };
			Spanned::new(Expression::Accessor(Spanned::new(accessor, span)).into(), span)
		}
		_ => expression,
	})
}

pub fn parse_expression<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, precedence: usize)
                            -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let mut context = super::parse_terminal(lexer, end_span)?;
	while let Ok(operator) = peek_operator(lexer, end_span) {
		match operator.precedence() > precedence {
			true => {
				lexer.next();
				context = parse_binder(lexer, end_span, context, operator)?
			},
			false => break,
		}
	}
	Ok(context)
}

pub fn parse_function_call<'a>(lexer: &mut PeekLexer<'a>, function: Spanned<Identifier<'a>>,
                               end_span: Span) -> ParserResult<'a, Spanned<FunctionCall<'a>>> {
	let end_error = Spanned::new(ParserError::ExpectedToken(Token::ParenthesisClose), end_span);
	expect!(lexer, end_span, ParenthesisOpen);

	let mut arguments = Vec::new();
	while lexer.peek().ok_or(end_error.clone())?.node != Token::ParenthesisClose {
		arguments.push(parse_expression_root(lexer, end_span)?);
		match lexer.peek().ok_or(end_error.clone())?.node {
			Token::ListSeparator => lexer.next(),
			_ => break,
		};
	}

	let evaluation_type = DataType::default();
	let byte_end = expect!(lexer, end_span, ParenthesisClose).byte_end;
	let span = Span::new(function.span.byte_start, byte_end);
	Ok(Spanned::new(FunctionCall { function, arguments, evaluation_type }, span))
}

pub fn parse_binder<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, context: Spanned<ExpressionNode<'a>>,
                        operator: Spanned<BinaryOperator>) -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let terminal = parse_expression(lexer, end_span, operator.precedence())?;
	let span = Span::new(context.span.byte_start, terminal.span.byte_end);
	let binder = BinaryOperation { left: context, right: terminal, operator };
	let expression = Expression::BinaryOperation(Spanned::new(binder, span));
	Ok(Spanned::new(expression.into(), span))
}

pub fn peek_operator<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                         -> ParserResult<'a, Spanned<BinaryOperator>> {
	let error = Spanned::new(ParserError::ExpectedOperator, end_span);
	let operator = lexer.peek().ok_or(error)?;

	Ok(Spanned::new(match operator.node {
		Token::Equal => BinaryOperator::Equal,
		Token::Add => BinaryOperator::Add,
		Token::Minus => BinaryOperator::Minus,
		Token::Multiply => BinaryOperator::Multiply,
		_ => return Err(Spanned::new(ParserError::ExpectedOperator, operator.span).into()),
	}, operator.span))
}
