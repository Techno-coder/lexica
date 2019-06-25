use crate::node::{BinaryOperation, BinaryOperator, Expression, Identifier};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_expression_root<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                 -> ParserResult<'a, Spanned<Expression<'a>>> {
	parse_expression(lexer, end_span, 0)
}

pub fn parse_expression<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, precedence: usize)
                            -> ParserResult<'a, Spanned<Expression<'a>>> {
	let mut context = parse_terminal(lexer, end_span)?;
	while let Ok(operator) = parse_operator(lexer, end_span) {
		match operator.precedence() > precedence {
			true => context = parse_binder(lexer, end_span, context, operator)?,
			false => break,
		}
	}
	Ok(context)
}

pub fn parse_terminal<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<Expression<'a>>> {
	let error = Spanned::new(ParserError::ExpectedExpression, end_span);
	let next_token = lexer.next().ok_or(error)?;
	Ok(Spanned::new(match next_token.node {
		Token::UnsignedInteger(integer) => Expression::LiteralInteger(integer as i64),
		Token::Identifier(identifier) => Expression::Variable(Identifier(identifier)),
		_ => return Err(Spanned::new(ParserError::ExpectedExpression, next_token.span).into()),
	}, next_token.span))
}

pub fn parse_binder<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, context: Spanned<Expression<'a>>,
                        operator: Spanned<BinaryOperator>) -> ParserResult<'a, Spanned<Expression<'a>>> {
	let terminal = parse_expression(lexer, end_span, operator.precedence())?;
	let span = Span::new(context.span.byte_start, terminal.span.byte_end);
	let binder = BinaryOperation { left: context, right: terminal, operator };
	Ok(Spanned::new(Expression::BinaryOperation(Box::new(binder)), span))
}

pub fn parse_operator<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Spanned<BinaryOperator>> {
	let error = Spanned::new(ParserError::ExpectedOperator, end_span);
	let operator = lexer.peek().ok_or(error)?;

	let operator = Spanned::new(match operator.node {
		Token::Equal => BinaryOperator::Equal,
		Token::Add => BinaryOperator::Add,
		Token::Minus => BinaryOperator::Minus,
		Token::Multiply => BinaryOperator::Multiply,
		_ => return Err(Spanned::new(ParserError::ExpectedOperator, operator.span).into()),
	}, operator.span);

	let _ = lexer.next();
	Ok(operator)
}

#[cfg(test)]
mod tests {
	use crate::parser::{end_span, Lexer};

	use super::*;

	#[test]
	fn test_precedence() {
		let text = "1 + 2 * 3\n";
		let (lexer, end_span) = (&mut Lexer::new(text), end_span(text));
		let expression = parse_expression_root(lexer, end_span).unwrap();

		let multiplication = BinaryOperation {
			left: Spanned::new(Expression::LiteralInteger(2), Span::new(4, 5)),
			right: Spanned::new(Expression::LiteralInteger(3), Span::new(8, 9)),
			operator: Spanned::new(BinaryOperator::Multiply, Span::new(6, 7)),
		};
		let multiplication = Expression::BinaryOperation(Box::new(multiplication));

		let root = BinaryOperation {
			left: Spanned::new(Expression::LiteralInteger(1), Span::new(0, 1)),
			right: Spanned::new(multiplication, Span::new(4, 9)),
			operator: Spanned::new(BinaryOperator::Add, Span::new(2, 3)),
		};
		assert_eq!(expression, Expression::BinaryOperation(Box::new(root)));
	}
}
