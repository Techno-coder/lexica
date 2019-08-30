use crate::interpreter::{Float, Integer, Primitive};
use crate::node::{BinaryOperation, BinaryOperator, DataType, Expression, ExpressionNode,
	FunctionCall, Identifier};
use crate::source::{Span, Spanned};

use super::{ParserError, ParserResult, PeekLexer, Token};

pub fn parse_expression_root<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                 -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	parse_expression(lexer, end_span, 0)
}

pub fn parse_expression<'a>(lexer: &mut PeekLexer<'a>, end_span: Span, precedence: usize)
                            -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let mut context = parse_terminal(lexer, end_span)?;
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
		_ => return Err(Spanned::new(ParserError::ExpectedExpression, next_token.span).into()),
	}
}

pub fn match_identifier_terminal<'a>(lexer: &mut PeekLexer<'a>, end_span: Span)
                                     -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
	let identifier = identifier!(lexer, end_span);
	let expression_variable = Expression::Variable(identifier.node.clone().into());
	Ok(match lexer.peek() {
		Some(token) if token.node == Token::ParenthesisOpen => {
			parse_function_call(lexer, identifier, end_span)?
		}
		_ => Spanned::new(expression_variable.into(), identifier.span),
	})
}

pub fn parse_function_call<'a>(lexer: &mut PeekLexer<'a>, function: Spanned<Identifier<'a>>,
                               end_span: Span) -> ParserResult<'a, Spanned<ExpressionNode<'a>>> {
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

	let function_call = FunctionCall { function, arguments, evaluation_type };
	let expression = Expression::FunctionCall(Spanned::new(function_call, span));
	Ok(Spanned::new(expression.into(), span))
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
