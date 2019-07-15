use std::iter::Peekable;

use crate::source::{Spanned, SplitSource};

use super::Token;

pub type PeekLexer<'a> = Peekable<Lexer<'a>>;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
	lexemes: SplitSource<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> PeekLexer<'a> {
		Lexer { lexemes: SplitSource::new(text) }.peekable()
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Spanned<Token<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		let (span, lexeme) = self.lexemes.next()?;
		if lexeme.starts_with("//") {
			return self.next();
		}

		let token = match lexeme {
			"fn" => Token::Function,
			"let" => Token::Binding,
			"drop" => Token::Drop,
			"loop" => Token::Loop,
			"when" => Token::When,
			"->" => Token::BlockSeparator,
			"(" => Token::ParenthesisOpen,
			")" => Token::ParenthesisClose,
			"{" => Token::BlockOpen,
			"}" => Token::BlockClose,
			":" => Token::VariableSeparator,
			"," => Token::ListSeparator,
			"~" => Token::MutableModifier,
			";" => Token::Terminator,
			"=" => Token::Assign,
			"==" => Token::Equal,
			"<" => Token::LessThan,
			"<=" => Token::LessThanEqual,
			"<=>" => Token::Swap,
			"=>" => Token::Implies,
			"+" => Token::Add,
			"-" => Token::Minus,
			"*" => Token::Multiply,
			"+=" => Token::AddAssign,
			"*=" => Token::MultiplyAssign,
			other => {
				if let Ok(integer) = other.parse::<u64>() {
					Token::UnsignedInteger(integer)
				} else if let Ok(integer) = other.parse::<i64>() {
					Token::SignedInteger(integer)
				} else if let Ok(float) = other.parse::<f64>() {
					Token::Float(float)
				} else if let Ok(boolean) = other.parse::<bool>() {
					Token::Boolean(boolean)
				} else {
					Token::Identifier(other)
				}
			}
		};
		Some(Spanned::new(token, span))
	}
}
