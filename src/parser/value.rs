use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Arithmetic, BinaryOperator, Expression, ExpressionKey, FunctionContext, Variable};
use crate::span::Spanned;

use super::ParserError;

pub fn root_value(context: &mut FunctionContext, lexer: &mut Lexer)
                  -> Result<ExpressionKey, Diagnostic> {
	value(context, lexer, 0)
}

fn value(context: &mut FunctionContext, lexer: &mut Lexer, precedence: usize)
         -> Result<ExpressionKey, Diagnostic> {
	let mut left = terminal(context, lexer)?;
	while token_precedence(&lexer.peek().node) > precedence {
		left = binder(context, lexer, left)?;
	}
	Ok(left)
}

fn binder(context: &mut FunctionContext, lexer: &mut Lexer, left: ExpressionKey)
          -> Result<ExpressionKey, Diagnostic> {
	let binder = lexer.next();
	let precedence = token_precedence(&binder.node);
	let right = value(context, lexer, precedence)?;
	let span = context[&left].span.merge(context[&right].span);

	match binder.node {
		Token::Add => {
			let operator = Spanned::new(BinaryOperator::Arithmetic(Arithmetic::Add), binder.span);
			Ok(context.register(Spanned::new(Expression::Binary(operator, left, right), span)))
		}
		Token::Minus => {
			let operator = Spanned::new(BinaryOperator::Arithmetic(Arithmetic::Minus), binder.span);
			Ok(context.register(Spanned::new(Expression::Binary(operator, left, right), span)))
		}
		Token::Multiply => {
			let operator = Spanned::new(BinaryOperator::Arithmetic(Arithmetic::Multiply), binder.span);
			Ok(context.register(Spanned::new(Expression::Binary(operator, left, right), span)))
		}
		Token::Equality => {
			let operator = Spanned::new(BinaryOperator::Equality, binder.span);
			Ok(context.register(Spanned::new(Expression::Binary(operator, left, right), span)))
		}
		_ => panic!("Invalid value binder: {:?}", binder.node),
	}
}

fn token_precedence(token: &Token) -> usize {
	match token {
		Token::Equality => 1,
		Token::Add | Token::Minus => 2,
		Token::Multiply => 3,
		_ => 0,
	}
}

fn terminal(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.peek();
	match &token.node {
		Token::BlockOpen => block(context, lexer),
		Token::ParenthesisOpen => {
			let pattern = super::pattern(lexer, &mut |lexer| root_value(context, lexer))?;
			Ok(context.register(pattern.map(|pattern| Expression::Pattern(pattern))))
		}
		_ => consume_terminal(context, lexer),
	}
}

fn consume_terminal(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.next();
	match token.node {
		Token::Unsigned(unsigned) => Ok(context
			.register(Spanned::new(Expression::Unsigned(unsigned), token.span))),
		Token::Signed(signed) => Ok(context
			.register(Spanned::new(Expression::Signed(signed), token.span))),
		Token::Truth(truth) => Ok(context
			.register(Spanned::new(Expression::Truth(truth), token.span))),
		Token::Identifier(identifier) => {
			let expression = Expression::Variable(Variable::new(identifier));
			Ok(context.register(Spanned::new(expression, token.span)))
		}
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
		if lexer.peek().node == Token::LineBreak {
			lexer.next();
			continue;
		}

		block.push(super::expression(context, lexer)?);
	}

	let span = initial_span.merge(super::expect(lexer, Token::BlockClose)?);
	Ok(context.register(Spanned::new(Expression::Block(block), span)))
}

