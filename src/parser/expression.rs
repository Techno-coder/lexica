use crate::node::{BinaryOperation, BinaryOperator, Expression, Identifier};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_expression_root<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                 -> ParserResult<'a, Expression<'a>> {
	parse_expression(lexer, end_span, 0)
}

pub fn parse_expression<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, precedence: usize)
                            -> ParserResult<'a, Expression<'a>> {
	let mut context = parse_terminal(lexer, end_span.clone())?;
	while let Ok(operator) = parse_operator(lexer, end_span.clone()) {
		match operator.precedence() > precedence {
			true => context = parse_binder(lexer, end_span.clone(), context, operator)?,
			false => break,
		}
	}
	Ok(context)
}

pub fn parse_terminal<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, Expression<'a>> {
	let error = Spanned::new(ParserError::ExpectedExpression, end_span);
	let next_token = lexer.next().ok_or(error)?;
	Ok(match next_token.node {
		Token::UnsignedInteger(integer) => Expression::LiteralInteger(integer as i64),
		Token::Identifier(identifier) => Expression::Variable(Identifier(identifier)),
		_ => return Err(Spanned::new(ParserError::ExpectedExpression, next_token.span).into()),
	})
}

pub fn parse_binder<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, context: Expression<'a>,
                        operator: BinaryOperator) -> ParserResult<'a, Expression<'a>> {
	let terminal = parse_expression(lexer, end_span, operator.precedence())?;
	let binder = BinaryOperation { left: context, right: terminal, operator };
	Ok(Expression::BinaryOperation(Box::new(binder)))
}

pub fn parse_operator<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                          -> ParserResult<'a, BinaryOperator> {
	let error = Spanned::new(ParserError::ExpectedOperator, end_span);
	let operator = lexer.peek().ok_or(error)?;

	let operator = match operator.node {
		Token::Equal => BinaryOperator::Equal,
		Token::Add => BinaryOperator::Add,
		Token::Minus => BinaryOperator::Minus,
		Token::Multiply => BinaryOperator::Multiply,
		_ => return Err(Spanned::new(ParserError::ExpectedOperator, operator.span.clone()).into()),
	};

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
			left: Expression::LiteralInteger(2),
			right: Expression::LiteralInteger(3),
			operator: BinaryOperator::Multiply,
		};
		let multiplication = Expression::BinaryOperation(Box::new(multiplication));

		let root = BinaryOperation {
			left: Expression::LiteralInteger(1),
			right: multiplication,
			operator: BinaryOperator::Add,
		};
		assert_eq!(expression, Expression::BinaryOperation(Box::new(root)));
	}
}
