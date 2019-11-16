use crate::declaration::{FunctionPath, StructurePath};
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token};
use crate::node::{Arithmetic, BinaryOperator, Execution, Expression, ExpressionKey,
	FunctionContext, Permission, UnaryOperator, Variable};
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
	if binder.node == Token::Dot {
		return projection(context, lexer, left);
	}

	let precedence = token_precedence(&binder.node);
	let right = value(context, lexer, precedence)?;
	let span = context[&left].span.merge(context[&right].span);

	let operator = match binder.node {
		Token::AngleRight => BinaryOperator::GreaterThan,
		Token::GreaterEqual => BinaryOperator::GreaterEqual,
		Token::AngleLeft => BinaryOperator::LessThan,
		Token::LessEqual => BinaryOperator::LessEqual,
		Token::Equality => BinaryOperator::Equality,
		Token::Add => BinaryOperator::Arithmetic(Arithmetic::Add),
		Token::Minus => BinaryOperator::Arithmetic(Arithmetic::Minus),
		Token::Multiply => BinaryOperator::Arithmetic(Arithmetic::Multiply),
		_ => panic!("Invalid value binder: {:?}", binder.node),
	};

	let operator = Spanned::new(operator, binder.span);
	Ok(context.register(Spanned::new(Expression::Binary(operator, left, right), span)))
}

fn token_precedence(token: &Token) -> usize {
	match token {
		Token::Equality => 1,
		Token::AngleLeft | Token::AngleRight => 2,
		Token::LessEqual | Token::GreaterEqual => 2,
		Token::Add | Token::Minus => 3,
		Token::Multiply => 4,
		Token::Dot => 5,
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
		Token::Compile => {
			lexer.next();
			let function_path = super::expression::path(lexer)?.map(|path| FunctionPath(path));
			function_call(context, lexer, Execution::Compile, function_path)
		}
		_ => consume_terminal(context, lexer),
	}
}

fn consume_terminal(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let token = lexer.next();
	match token.node {
		Token::Integer(integer) => Ok(context
			.register(Spanned::new(Expression::Integer(integer), token.span))),
		Token::Truth(truth) => Ok(context
			.register(Spanned::new(Expression::Truth(truth), token.span))),
		Token::Minus => unary(context, lexer,
			Spanned::new(UnaryOperator::Negate, token.span)),
		Token::Asterisk => unary(context, lexer,
			Spanned::new(UnaryOperator::Dereference, token.span)),
		Token::Reference => unary(context, lexer,
			Spanned::new(UnaryOperator::Reference(Permission::Shared), token.span)),
		Token::Unique => {
			super::expect(lexer, Token::Reference)?;
			let operator = UnaryOperator::Reference(Permission::Unique);
			unary(context, lexer, Spanned::new(operator, token.span))
		}
		Token::Identifier(identifier) => match lexer.peek().node {
			Token::ParenthesisOpen | Token::PathSeparator => {
				let identifier = Spanned::new(identifier, token.span);
				let path = super::expression::path_identifier(lexer, identifier)?;
				match lexer.peek().node {
					Token::ParenthesisOpen => {
						let function_path = path.map(|path| FunctionPath(path));
						function_call(context, lexer, Execution::Runtime, function_path)
					}
					Token::Separator => super::structure::literal(context, lexer,
						path.map(|path| StructurePath(path))),
					_ => {
						let token = lexer.next();
						let error = ParserError::ExpectedPathAssociation(token.node);
						Err(Diagnostic::new(Spanned::new(error, token.span)))
					}
				}
			}
			Token::Separator => {
				let recover = &mut lexer.clone();
				let identifier = Spanned::new(identifier, token.span);
				let path = super::expression::path_identifier(recover, identifier.clone())?;
				let structure_path = path.map(|path| StructurePath(path));

				Ok(match super::structure::literal(context, recover, structure_path) {
					Err(_) => context.register(identifier.map(|identifier|
						Expression::Variable(Variable::new(identifier)))),
					Ok(literal) => {
						std::mem::swap(lexer, recover);
						literal
					}
				})
			}
			_ => {
				let expression = Expression::Variable(Variable::new(identifier));
				Ok(context.register(Spanned::new(expression, token.span)))
			}
		},
		other => {
			let error = ParserError::ExpectedExpression(other.clone());
			Err(Diagnostic::new(Spanned::new(error, token.span)))
		}
	}
}

fn unary(context: &mut FunctionContext, lexer: &mut Lexer,
         operator: Spanned<UnaryOperator>) -> Result<ExpressionKey, Diagnostic> {
	let expression = root_value(context, lexer)?;
	let span = operator.span.merge(context[&expression].span);
	Ok(context.register(Spanned::new(Expression::Unary(operator, expression), span)))
}

fn function_call(context: &mut FunctionContext, lexer: &mut Lexer, execution: Execution,
                 function_path: Spanned<FunctionPath>) -> Result<ExpressionKey, Diagnostic> {
	let initial_span = function_path.span;
	let arguments = arguments(context, lexer)?;
	let function_call = Expression::FunctionCall(function_path, arguments.node, execution);
	Ok(context.register(Spanned::new(function_call, initial_span.merge(arguments.span))))
}

fn projection(context: &mut FunctionContext, lexer: &mut Lexer,
              expression: ExpressionKey) -> Result<ExpressionKey, Diagnostic> {
	let identifier = super::identifier(lexer).map_err(|diagnostic|
		diagnostic.note("In parsing a field or method call"))?;
	let span = identifier.span;

	let expression = Spanned::new(match lexer.peek().node {
		// TODO: Implement parse of method call
		Token::ParenthesisOpen => unimplemented!(),
		_ => Expression::Field(expression, identifier),
	}, span);
	Ok(context.register(expression))
}

fn arguments(context: &mut FunctionContext, lexer: &mut Lexer)
             -> Result<Spanned<Vec<ExpressionKey>>, Diagnostic> {
	let mut arguments = Vec::new();
	let initial_span = super::expect(lexer, Token::ParenthesisOpen)?;
	super::list(lexer, Token::ParenthesisClose, Token::ListSeparator, &mut |lexer|
		Ok(arguments.push(root_value(context, lexer).map_err(|diagnostic|
			diagnostic.note("In parsing an argument"))?)))?;
	let span = initial_span.merge(super::expect(lexer, Token::ParenthesisClose)?);
	Ok(Spanned::new(arguments, span))
}

fn block(context: &mut FunctionContext, lexer: &mut Lexer) -> Result<ExpressionKey, Diagnostic> {
	let mut block = Vec::new();
	let initial_span = super::expect(lexer, Token::BlockOpen)?;
	while lexer.peek().node != Token::BlockClose {
		block.push(super::expression(context, lexer)?);
		super::skip(lexer, &Token::LineBreak);
	}

	let span = initial_span.merge(super::expect(lexer, Token::BlockClose)?);
	Ok(context.register(Spanned::new(Expression::Block(block), span)))
}

