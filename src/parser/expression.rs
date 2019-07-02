use crate::interpreter::{Integer, Primitive};
use crate::node::{BinaryOperation, BinaryOperator, Expression, FunctionCall, Identifier};
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
		Token::Identifier(identifier) => {
			let identifier = Spanned::new(Identifier(identifier), next_token.span);
			return match_identifier_terminal(lexer, identifier, end_span);
		}
		Token::UnsignedInteger(integer) => {
			let integer = Integer::new_unsigned(integer);
			Expression::Primitive(Primitive::Integer(integer))
		}
		_ => return Err(Spanned::new(ParserError::ExpectedExpression, next_token.span).into()),
	}, next_token.span))
}

pub fn match_identifier_terminal<'a>(lexer: &mut PeekLexer<'a>, identifier: Spanned<Identifier<'a>>,
                                     end_span: Span) -> ParserResult<'a, Spanned<Expression<'a>>> {
	Ok(match lexer.peek() {
		Some(token) => match token.node {
			Token::ParenthesisOpen => parse_function_call(lexer, identifier, end_span)?,
			_ => Spanned::new(Expression::Variable(identifier.node), identifier.span),
		}
		None => Spanned::new(Expression::Variable(identifier.node), identifier.span),
	})
}

pub fn parse_function_call<'a>(lexer: &mut PeekLexer<'a>, function: Spanned<Identifier<'a>>,
                               end_span: Span) -> ParserResult<'a, Spanned<Expression<'a>>> {
	let end_error = Spanned::new(ParserError::ExpectedToken(Token::ParenthesisClose), end_span);
	expect!(lexer, end_span, ParenthesisOpen);

	let mut arguments = Vec::new();
	let byte_end = loop {
		arguments.push(parse_expression_root(lexer, end_span)?);
		match lexer.next() {
			Some(token) => match token.node {
				Token::ListSeparator => continue,
				Token::ParenthesisClose => break token.span.byte_end,
				_ => return Err(end_error.into()),
			}
			None => return Err(end_error.into()),
		}
	};

	let span = Span::new(function.span.byte_start, byte_end);
	let function_call = Box::new(FunctionCall { function, arguments });
	Ok(Spanned::new(Expression::FunctionCall(function_call), span))
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
		assert_eq!(expression.node, Expression::BinaryOperation(Box::new(root)));
	}
}
