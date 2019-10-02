use std::fmt;
use std::sync::Arc;

use crate::declaration::FunctionPath;
use crate::error::{CompileError, Diagnostic};
use crate::lexer::{Lexer, Token};
use crate::node::Pattern;
use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum ParserError {
	UndefinedFunction(Arc<FunctionPath>),
	ExpectedExpression(Token),
	ExpectedIdentifier(Token),
	ExpectedToken(Token, Token),
}

impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParserError::UndefinedFunction(path) =>
				write!(f, "Function: {}, is not defined", path),
			ParserError::ExpectedExpression(token) =>
				write!(f, "Expected an expression, instead got: {:?}", token),
			ParserError::ExpectedIdentifier(token) =>
				write!(f, "Expected an identifier, instead got: {:?}", token),
			ParserError::ExpectedToken(expected, token) =>
				write!(f, "Expected token: {:?}, instead got: {:?}", expected, token),
		}
	}
}

impl From<ParserError> for CompileError {
	fn from(error: ParserError) -> Self {
		CompileError::Parser(error)
	}
}

pub fn identifier(lexer: &mut Lexer) -> Result<Spanned<Arc<str>>, Diagnostic> {
	let token = lexer.next();
	match token.node {
		Token::Identifier(identifier) => Ok(Spanned::new(identifier, token.span)),
		_ => Err(Diagnostic::new(token.map(|token| ParserError::ExpectedIdentifier(token))))
	}
}

pub fn expect(lexer: &mut Lexer, expected: Token) -> Result<Span, Diagnostic> {
	let token = lexer.next();
	match token.node == expected {
		false => Err(Diagnostic::new(token.map(|token| ParserError::ExpectedToken(expected, token)))),
		true => Ok(token.span),
	}
}

pub fn pattern<F, T>(lexer: &mut Lexer, terminal: &mut F) -> Result<Spanned<Pattern<T>>, Diagnostic>
	where F: FnMut(&mut Lexer) -> Result<T, Diagnostic> {
	let token = lexer.peek();
	let token_span = token.span;
	Ok(match token.node {
		Token::ParenthesisOpen => {
			lexer.next();
			let mut elements = Vec::new();
			while lexer.peek().node != Token::ParenthesisClose {
				elements.push(pattern(lexer, terminal).map_err(|diagnostic|
					diagnostic.note("In parsing a tuple pattern"))?.node);
				match lexer.peek().node {
					Token::ListSeparator => lexer.next(),
					_ => break,
				};
			}

			let end_span = super::expect(lexer, Token::ParenthesisClose)?;
			Spanned::new(Pattern::Tuple(elements), token_span.merge(end_span))
		}
		Token::Wildcard => Spanned::new(Pattern::Wildcard, lexer.next().span),
		_ => Spanned::new(Pattern::Terminal(terminal(lexer)?), token_span),
	})
}

