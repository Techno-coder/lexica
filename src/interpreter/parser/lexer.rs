use crate::source::{Spanned, SplitWhitespace};

use super::Token;

/// Parses a string into separate tokens.
#[derive(Debug)]
pub struct Lexer<'a> {
	lexemes: SplitWhitespace<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		let lexemes = SplitWhitespace::new(text);
		Self { lexemes }
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Spanned<Token<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		let (span, lexeme) = self.lexemes.next()?;
		let (left_index, left) = lexeme.char_indices().next().unwrap();
		let (right_index, right) = lexeme.char_indices().last().unwrap();

		let right_rest = || &lexeme[1..];
		let left_rest = || &lexeme[..right_index];
		let middle = || &lexeme[1..right_index];

		let token = match (left, right) {
			('@', _) => Token::Annotation(right_rest()),
			('+', ':') => Token::FunctionLabel(middle()),
			('-', '^') => Token::ReverseLabel(middle()),
			('.', ':') => Token::LocalLabel(middle()),
			(_, ':') => Token::Label(left_rest()),
			('+', '\'') => Token::ReverseOnAdvancing(middle()),
			('-', '\'') => Token::ReverseOnReversing(middle()),
			(_, '\'') => Token::Reversed(left_rest()),
			('+', _) => Token::AdvanceOnAdvancing(right_rest()),
			('-', _) => Token::AdvanceOnReversing(right_rest()),
			('#', _) => Token::Comment(lexeme),
			('*', _) if lexeme.len() == 1 => Token::ReversalHint,
			('=', _) if lexeme.len() == 1 => Token::Equal,
			('<', _) if lexeme.len() == 1 => Token::LessThan,
			('>', _) if lexeme.len() == 1 => Token::GreaterThan,
			('<', '=') if lexeme.len() == 2 => Token::LessThanEqual,
			('>', '=') if lexeme.len() == 2 => Token::GreaterThanEqual,
			_ => {
				if let Ok(unsigned) = lexeme.parse::<u64>() {
					Token::UnsignedInteger(unsigned)
				} else if let Ok(signed) = lexeme.parse::<i64>() {
					Token::SignedInteger(signed)
				} else if let Ok(float) = lexeme.parse::<f64>() {
					Token::Float(float)
				} else {
					Token::Identifier(lexeme)
				}
			}
		};
		Some(Spanned::new(token, span))
	}
}
