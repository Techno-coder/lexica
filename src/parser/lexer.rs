use std::iter::Peekable;

use crate::source::{Span, Spanned, SplitSource};

use super::Token;

pub type PeekLexer<'a> = Peekable<Lexer<'a>>;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
	lexemes: SplitSource<'a>,
	stack: Option<Spanned<Token<'a>>>,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> PeekLexer<'a> {
		Lexer {
			lexemes: SplitSource::new(text),
			stack: None,
		}.peekable()
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Spanned<Token<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		let (mut span, mut lexeme) = match self.stack.take() {
			Some(token) => return Some(token),
			None => self.lexemes.next()?,
		};

		if lexeme.starts_with("//") {
			return self.next();
		}

		if lexeme.ends_with(";") && lexeme.len() > 1 {
			let stack_span = Span::new(span.byte_end - 1, span.byte_end);
			self.stack = Some(Spanned::new(Token::Terminator, stack_span));
			span = Span::new(span.byte_start, span.byte_end - 1);
			lexeme = &lexeme[..(lexeme.len() - 1)];
		}

		let token = match lexeme {
			"fn" => Token::Function,
			"let" => Token::Binding,
			"drop" => Token::Drop,
			"loop" => Token::Loop,
			"->" => Token::ReturnSeparator,
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
			"()" => {
				let stack_span = Span::new(span.byte_start + 1, span.byte_end);
				self.stack = Some(Spanned::new(Token::ParenthesisClose, stack_span));
				let token_span = Span::new(span.byte_start, span.byte_start + 1);
				return Some(Spanned::new(Token::ParenthesisOpen, token_span));
			}
			other => {
				if let Ok(integer) = other.parse::<u64>() {
					Token::UnsignedInteger(integer)
				} else {
					Token::Identifier(other)
				}
			}
		};
		Some(Spanned::new(token, span))
	}
}
