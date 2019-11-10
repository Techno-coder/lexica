use std::fmt;
use std::sync::Arc;

use crate::declaration::{FunctionPath, StructurePath};
use crate::error::{CompileError, Diagnostic};
use crate::lexer::{Lexer, Token};
use crate::node::Pattern;
use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum ParserError {
	UndefinedFunction(Arc<FunctionPath>),
	UndefinedStructure(Arc<StructurePath>),
	ExpectedExpression(Token),
	ExpectedIdentifier(Token),
	ExpectedToken(Token, Token),
	ExpectedExpressionTerminator(Token),
	ExpectedPathAssociation(Token),
	DuplicateField(Arc<str>),
}

impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParserError::UndefinedFunction(path) =>
				write!(f, "Function: {}, is not defined", path),
			ParserError::UndefinedStructure(path) =>
				write!(f, "Structure: {}, is not defined", path),
			ParserError::ExpectedExpression(token) =>
				write!(f, "Expected an expression, instead got: {:?}", token),
			ParserError::ExpectedIdentifier(token) =>
				write!(f, "Expected an identifier, instead got: {:?}", token),
			ParserError::ExpectedToken(expected, token) =>
				write!(f, "Expected token: {:?}, instead got: {:?}", expected, token),
			ParserError::ExpectedExpressionTerminator(token) =>
				write!(f, "Expected line break or mutation operator, instead got: {:?}", token),
			ParserError::ExpectedPathAssociation(token) =>
				write!(f, "Expected a function call or structure literal instead got: {:?}", token),
			ParserError::DuplicateField(field) =>
				write!(f, "Field with identifier: {}, has already been defined", field),
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
		false => Err(Diagnostic::new(token.map(|token|
			ParserError::ExpectedToken(expected, token)))),
		true => Ok(token.span),
	}
}

/// Expects a specified token but does not consume it.
pub fn expect_peek(lexer: &mut Lexer, expected: Token) -> Result<Span, Diagnostic> {
	let token = lexer.peek();
	match token.node == expected {
		false => Err(Diagnostic::new(lexer.next().map(|token|
			ParserError::ExpectedToken(expected, token)))),
		true => Ok(token.span),
	}
}

/// Ignores a contiguous sequence of tokens of the specified variant.
pub fn skip(lexer: &mut Lexer, token: &Token) {
	while &lexer.peek().node == token {
		lexer.next();
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
			let mut trailing_separator = false;
			while lexer.peek().node != Token::ParenthesisClose {
				trailing_separator = false;
				elements.push(pattern(lexer, terminal).map_err(|diagnostic|
					diagnostic.note("In parsing a tuple pattern"))?.node);
				match lexer.peek().node {
					Token::ListSeparator => {
						trailing_separator = true;
						lexer.next();
					}
					_ => break,
				}
			}

			let end_span = super::expect(lexer, Token::ParenthesisClose)?;
			Spanned::new(match trailing_separator {
				false if elements.len() == 1 => match elements.pop().unwrap() {
					Pattern::Terminal(terminal) => Pattern::Terminal(terminal),
					_ => Pattern::Tuple(elements),
				},
				_ => Pattern::Tuple(elements),
			}, token_span.merge(end_span))
		}
		Token::Wildcard => Spanned::new(Pattern::Wildcard, lexer.next().span),
		_ => Spanned::new(Pattern::Terminal(terminal(lexer)?), token_span),
	})
}

