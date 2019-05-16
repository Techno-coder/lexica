use std::convert::TryFrom;
use std::iter::Peekable;

use crate::source::{Span, Spanned};

use super::{Annotation, AnnotationStore, Annotator, Argument, Direction, Lexer,
            OperationStore, ParserError, ParserResult, Token, TranslationInstruction};

/// A singular unit of textual code.
#[derive(Debug, Clone)]
pub enum Element<'a> {
	Annotation(Annotation<'a>),
	Instruction(TranslationInstruction<'a>),
	Function(&'a str),
	Label(&'a str),
}

/// Parses tokens into elements.
pub struct ElementParser<'a> {
	lexer: Peekable<Lexer<'a>>,
	annotations: &'a AnnotationStore,
	operations: &'a OperationStore,
	function: Option<Span>,
}

impl<'a> ElementParser<'a> {
	pub fn new(text: &'a str, annotations: &'a AnnotationStore,
	           operations: &'a OperationStore) -> Self {
		let lexer = Lexer::new(text).peekable();
		Self { lexer, annotations, operations, function: None }
	}

	/// Advances the lexer until a valid state is reached and returns
	/// the provided error.
	fn discard<T>(&mut self, span: Span, error: ParserError<'a>) -> ParserResult<'a, T> {
		while let Some(token) = self.lexer.peek() {
			let _ = match token.node.element_delimiter() {
				true => break,
				false => self.lexer.next(),
			};
		}
		Err(Spanned::new(error, span))
	}

	/// Parses an annotation from the subsequent tokens.
	fn annotation(&mut self, span: Span, identifier: &'a str)
	              -> Option<ParserResult<'a, Spanned<Element<'a>>>> {
		Some(match self.annotations.get(identifier) {
			Some(annotation) => match self.annotation_arguments(span.clone(), annotation) {
				Ok(arguments) => {
					let annotation = Annotation { identifier, arguments };
					Ok(Spanned::new(Element::Annotation(annotation), span))
				}
				Err(error) => return Some(self.discard(error.span, error.node)),
			}
			None => self.discard(span, ParserError::InvalidAnnotation(identifier)),
		})
	}

	/// Parses the annotation arguments from the subsequent tokens.
	fn annotation_arguments(&mut self, span: Span, annotation: &Annotator)
	                        -> ParserResult<'a, Vec<Spanned<Argument<'a>>>> {
		let mut arguments: Vec<Spanned<Argument>> = Vec::new();
		for _ in 0..annotation.arity() {
			match self.lexer.next() {
				Some(argument) => match Argument::try_from(argument.node) {
					Ok(argument_type) => arguments.push(Spanned::new(argument_type, argument.span)),
					Err(error) => return Err(Spanned::new(error, argument.span)),
				}
				None => return Err(Spanned::new(ParserError::EndOfInput, span.clone())),
			}
		}
		Ok(arguments)
	}

	/// Parses an instruction from the subsequent tokens.
	fn instruction(&mut self, span: Span, identifier: &'a str, direction: Direction,
	               polarization: Option<Direction>) -> ParserResult<'a, Spanned<Element<'a>>> {
		self.expect_function_context(span.clone())?;

		let error = ParserError::InvalidOperation(identifier);
		let (operation, _) = self.operations.get(identifier)
			.ok_or_else(|| self.discard::<!>(span.clone(), error).unwrap_err())?;

		let arity = self.operations.arity(identifier).unwrap();
		let arguments = (0..arity).map(|_| self.lexer.next()).collect::<Option<Vec<_>>>();

		let error = || self.discard::<!>(span.clone(), ParserError::EndOfInput).unwrap_err();
		let operands = arguments.ok_or_else(error)?;
		let instruction = TranslationInstruction { operation, operands, direction, polarization };
		Ok(Spanned::new(Element::Instruction(instruction), span))
	}

	/// Parses a reversal hint from the subsequent tokens.
	/// If there are consecutive reversal hint tokens, only one is produced.
	fn reversal_hint(&mut self, span: Span) -> ParserResult<'a, Spanned<Element<'a>>> {
		self.expect_function_context(span.clone())?;
		while let Some(token) = self.lexer.peek() {
			let _ = match token.node {
				Token::ReversalHint => self.lexer.next(),
				_ => break,
			};
		}
		self.instruction(span, "*", Direction::Advance, None)
	}

	/// Parses a function identifier.
	/// Verifies that only one function is in context.
	fn function(&mut self, span: Span, identifier: &'a str) -> ParserResult<'a, Spanned<Element<'a>>> {
		if self.function.is_some() {
			return self.discard(span, ParserError::DuplicateFunctionContext);
		}

		match self.lexer.next() {
			Some(token) => match token.node {
				Token::FunctionOpen => (),
				other => return self.discard(span, ParserError::UnexpectedToken(other)),
			}
			None => return Err(Spanned::new(ParserError::EndOfInput, span.clone())),
		}

		self.function = Some(span.clone());
		Ok(Spanned::new(Element::Function(identifier), span))
	}

	/// Parses a function close delimiter.
	fn function_close(&mut self, span: Span) -> ParserResult<'a, ()> {
		match self.function {
			Some(_) => Ok(self.function = None),
			None => self.discard(span, ParserError::UnmatchedFunctionClose),
		}
	}

	/// Helper function that returns an error if a function is not in context.
	fn expect_function_context(&mut self, span: Span) -> ParserResult<'a, ()> {
		match self.function.is_none() {
			true => self.discard(span, ParserError::FunctionMissingContext),
			false => Ok(()),
		}
	}
}

impl<'a> Iterator for ElementParser<'a> {
	type Item = ParserResult<'a, Spanned<Element<'a>>>;

	fn next(&mut self) -> Option<Self::Item> {
		let token = match self.lexer.next() {
			Some(token) => token,
			None => return match self.function.take() {
				Some(function_span) => {
					let error = Spanned::new(ParserError::UnmatchedFunctionOpen, function_span);
					Some(Err(error))
				}
				None => None,
			}
		};

		Some(match token.node {
			Token::Annotation(identifier) => return self.annotation(token.span, identifier),
			Token::Function(identifier) => self.function(token.span, identifier),
			Token::FunctionClose => match self.function_close(token.span) {
				Ok(()) => return self.next(),
				Err(error) => Err(error),
			},
			Token::Identifier(identifier) => self
				.instruction(token.span, identifier, Direction::Advance, None),
			Token::Reversed(identifier) => self
				.instruction(token.span, identifier, Direction::Reverse, None),
			Token::AdvanceOnAdvancing(identifier) => self
				.instruction(token.span, identifier, Direction::Advance, Some(Direction::Advance)),
			Token::AdvanceOnReversing(identifier) => self
				.instruction(token.span, identifier, Direction::Advance, Some(Direction::Reverse)),
			Token::ReverseOnAdvancing(identifier) => self
				.instruction(token.span, identifier, Direction::Reverse, Some(Direction::Advance)),
			Token::ReverseOnReversing(identifier) => self
				.instruction(token.span, identifier, Direction::Reverse, Some(Direction::Reverse)),
			Token::Label(label) => Ok(Spanned::new(Element::Label(label), token.span)),
			Token::ReversalHint => self.reversal_hint(token.span),
			Token::Comment(_) => return self.next(),
			other => self.discard(token.span, ParserError::UnexpectedToken(other)),
		})
	}
}
