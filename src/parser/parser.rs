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
	ExpectedStructureTerminator(Token),
	ExpectedPathAssociation(Token),
	DuplicateField(Arc<str>),
	FunctionSelfVariable,
	BindingSelfVariable,
	SelfVariablePosition,
}

impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParserError::UndefinedFunction(path) =>
				write!(f, "Function: {}, is not defined", path),
			ParserError::UndefinedStructure(path) =>
				write!(f, "Structure: {}, is not defined", path),
			ParserError::ExpectedExpression(token) =>
				write!(f, "Expected an expression, instead got token: {:?}", token),
			ParserError::ExpectedIdentifier(token) =>
				write!(f, "Expected an identifier, instead got token: {:?}", token),
			ParserError::ExpectedToken(expected, token) =>
				write!(f, "Expected token: {:?}, instead got token: {:?}", expected, token),
			ParserError::ExpectedExpressionTerminator(token) =>
				write!(f, "Expected line break or mutation operator, instead got token: {:?}", token),
			ParserError::ExpectedStructureTerminator(token) =>
				write!(f, "Expected separator or template declaration, instead got token: {:?}", token),
			ParserError::ExpectedPathAssociation(token) =>
				write!(f, "Expected a function call or structure literal instead got token: {:?}", token),
			ParserError::DuplicateField(field) =>
				write!(f, "Field with identifier: {}, has already been defined", field),
			ParserError::FunctionSelfVariable =>
				write!(f, "Variable: self, cannot appear in function that is not in a definition"),
			ParserError::BindingSelfVariable =>
				write!(f, "Variable: self, cannot be bound in a function except in parameters"),
			ParserError::SelfVariablePosition =>
				write!(f, "Self variable must appear as first parameter in a method"),
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

/// Ignores a contiguous sequence of tokens of the specified variant.
pub fn skip(lexer: &mut Lexer, token: &Token) {
	while &lexer.peek().node == token {
		lexer.next();
	}
}

/// Parses a separated and terminated list of elements.
/// Returns true if the list separator was trailing.
/// The list terminator is not consumed.
pub fn list<T, F, E>(lexer: &mut Lexer, terminator: Token, separator: Token, element: &mut F)
                     -> Result<(Vec<T>, bool), E> where F: FnMut(&mut Lexer) -> Result<T, E> {
	let mut trailing = false;
	let mut elements = Vec::new();
	while lexer.peek().node != terminator {
		trailing = false;
		elements.push(element(lexer)?);
		match lexer.peek().node == separator {
			false => break,
			true => {
				trailing = true;
				lexer.next();
			}
		}
	}
	Ok((elements, trailing))
}

pub fn pattern<F, T>(lexer: &mut Lexer, terminal: &mut F) -> Result<Spanned<Pattern<T>>, Diagnostic>
	where F: FnMut(&mut Lexer) -> Result<T, Diagnostic> {
	let token = lexer.peek();
	let token_span = token.span;
	Ok(match token.node {
		Token::ParenthesisOpen => {
			let (mut elements, trailing) = list(lexer.consume(), Token::ParenthesisClose,
				Token::ListSeparator, &mut |lexer| Ok(pattern(lexer, terminal).map_err(|diagnostic|
					diagnostic.note("In parsing a tuple pattern"))?.node))?;
			let end_span = super::expect(lexer, Token::ParenthesisClose)?;
			Spanned::new(match trailing {
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

