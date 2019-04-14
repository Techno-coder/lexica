use crate::source::SplitWhitespace;

use super::{Token, TokenType};

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
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let (span, lexeme) = self.lexemes.next()?;
		let (left_index, left) = lexeme.char_indices().next().unwrap();
		let (right_index, right) = lexeme.char_indices().last().unwrap();

		let right_rest = || &lexeme[1..];
		let left_rest = || &lexeme[..right_index];
		let middle = || &lexeme[1..right_index];

		let token_type = match (left, right) {
			('@', _) => TokenType::Annotation(right_rest()),
			('+', ':') => TokenType::FunctionLabel(middle()),
			('-', '^') => TokenType::ReverseLabel(middle()),
			('.', ':') => TokenType::LocalLabel(middle()),
			(_, ':') => TokenType::Label(left_rest()),
			('+', _) => TokenType::Advance(right_rest()),
			('-', _) => TokenType::Reverse(right_rest()),
			('#', _) => TokenType::Comment(lexeme),
			('*', _) if lexeme.len() == 1 => TokenType::ReversalHint,
			('=', _) if lexeme.len() == 1 => TokenType::Equal,
			('<', _) if lexeme.len() == 1 => TokenType::LessThan,
			('>', _) if lexeme.len() == 1 => TokenType::GreaterThan,
			('<', '=') if lexeme.len() == 2 => TokenType::LessThanEqual,
			('>', '=') if lexeme.len() == 2 => TokenType::GreaterThanEqual,
			_ => {
				if let Ok(unsigned) = lexeme.parse::<u64>() {
					TokenType::UnsignedInteger(unsigned)
				} else if let Ok(signed) = lexeme.parse::<i64>() {
					TokenType::SignedInteger(signed)
				} else if let Ok(float) = lexeme.parse::<f64>() {
					TokenType::Float(float)
				} else {
					TokenType::Identifier(lexeme)
				}
			}
		};
		Some(Token { span, token_type })
	}
}
