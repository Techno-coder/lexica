use std::convert::TryFrom;
use std::iter::Peekable;

use crate::source::{Span, Spanned};

use super::{Annotation, AnnotationMap, AnnotationType, Argument, Direction, Element,
            Lexer, OperationIdentifier, ParserError, ParserResult, Token, TranslationInstruction};

/// Parses tokens into elements.
#[derive(Debug)]
pub struct ElementParser<'a> {
	lexer: Peekable<Lexer<'a>>,
	annotation_map: &'a AnnotationMap,
}

impl<'a> ElementParser<'a> {
	pub fn new(text: &'a str, annotation_map: &'a AnnotationMap) -> Self {
		let lexer = Lexer::new(text).peekable();
		Self { lexer, annotation_map }
	}

	/// Advances the lexer until a valid state is reached and returns
	/// the provided error.
	fn discard<T>(&mut self, span: Span, error: ParserError<'a>) -> ParserResult<'a, T> {
		while let Some(token) = self.lexer.peek() {
			if token.node.element_delimiter() {
				break;
			} else {
				let _ = self.lexer.next();
			}
		}
		Err(Spanned::new(error, span))
	}

	/// Parses an annotation from the subsequent tokens.
	fn annotation(&mut self, span: Span, identifier: &'a str)
	              -> Option<ParserResult<'a, Spanned<Element<'a>>>> {
		Some(match self.annotation_map.get(identifier) {
			Some(annotation) => {
				match self.annotation_arguments(span.clone(), annotation) {
					Ok(arguments) => {
						let annotation = Annotation { identifier, arguments };
						Ok(Spanned::new(Element::Annotation(annotation), span))
					}
					Err(error) => return Some(self.discard(error.span, error.node)),
				}
			}
			None => self.discard(span, ParserError::InvalidAnnotation(identifier)),
		})
	}

	/// Parses the annotation arguments from the subsequent tokens.
	fn annotation_arguments(&mut self, span: Span, annotation: &AnnotationType)
	                        -> ParserResult<'a, Vec<Spanned<Argument<'a>>>> {
		let mut arguments: Vec<Spanned<Argument>> = Vec::new();
		for _ in 0..annotation.arity() {
			match self.lexer.next() {
				Some(argument) => {
					let argument_type = Argument::try_from(argument.node);
					if let Err(error) = argument_type {
						return Err(Spanned::new(error, argument.span));
					}
					arguments.push(Spanned::new(argument_type.unwrap(), argument.span));
				}
				None => return Err(Spanned::new(ParserError::EndOfInput, span.clone())),
			}
		}
		Ok(arguments)
	}

	/// Parses an instruction from the subsequent tokens.
	/// Additionally, verifies the instruction directions and polarizations are valid.
	fn instruction(&mut self, span: Span, identifier: &'a str, direction: Direction,
	               polarization: Option<Direction>) -> ParserResult<'a, Spanned<Element<'a>>> {
		let operation = match OperationIdentifier::parse(identifier) {
			Some(operation) => match (operation.reversible(), direction, polarization) {
				(false, Direction::Reverse, _) => return self
					.discard(span, ParserError::IrreversibleOperation(identifier)),
				(false, _, None) => return self.discard(span, ParserError::MissingPolarization(identifier)),
				_ => operation,
			},
			None => return self.discard(span.clone(), ParserError::InvalidOperation(identifier)),
		};

		let arguments = (0..operation.arity())
			.map(|_| self.lexer.next())
			.collect::<Option<Vec<_>>>();

		let arguments = match arguments {
			Some(arguments) => arguments,
			None => return self.discard(span.clone(), ParserError::EndOfInput),
		};

		let instruction = TranslationInstruction { operation, operands: arguments, direction, polarization };
		Ok(Spanned::new(Element::Instruction(instruction), span))
	}

	/// Parses a reversal hint from the subsequent tokens.
	/// If there are consecutive reversal hint tokens, only one is produced.
	fn reversal_hint(&mut self, span: Span) -> ParserResult<'a, Spanned<Element<'a>>> {
		while let Some(token) = self.lexer.peek() {
			let _ = match token.node {
				Token::ReversalHint => self.lexer.next(),
				_ => break,
			};
		}
		Ok(Spanned::new(Element::ReversalHint, span))
	}
}

impl<'a> Iterator for ElementParser<'a> {
	type Item = ParserResult<'a, Spanned<Element<'a>>>;

	fn next(&mut self) -> Option<Self::Item> {
		let token = self.lexer.next()?;
		Some(match token.node {
			Token::Annotation(identifier) => return self.annotation(token.span, identifier),
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
			Token::LocalLabel(label) => Ok(Spanned::new(Element::LocalLabel(label), token.span)),
			Token::FunctionLabel(label) => Ok(Spanned::new(Element::FunctionLabel(label), token.span)),
			Token::ReverseLabel(label) => Ok(Spanned::new(Element::ReverseLabel(label), token.span)),
			Token::ReversalHint => self.reversal_hint(token.span),
			Token::Comment(_) => return self.next(),
			other => self.discard(token.span, ParserError::UnexpectedToken(other)),
		})
	}
}
