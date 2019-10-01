use crate::source::SourceKey;
use crate::span::Spanned;

use super::direct_lexer::DirectLexer;
use super::Token;

/// Adds one token lookahead to `DirectLexer`.
#[derive(Debug)]
pub struct Lexer<'a> {
	lexer: DirectLexer<'a>,
	token: Option<Spanned<Token>>,
	byte_offset: usize,
}

impl<'a> Lexer<'a> {
	/// Creates a new `Lexer` instance.
	/// `byte_offset` specifies where to start lexing.
	pub fn new(string: &'a str, byte_offset: usize, source_key: SourceKey) -> Self {
		Lexer {
			lexer: DirectLexer::new(&string[byte_offset..], source_key),
			token: None,
			byte_offset,
		}
	}

	pub fn next(&mut self) -> Spanned<Token> {
		let lexer = &mut self.lexer;
		let byte_offset = self.byte_offset;
		self.token.take().unwrap_or_else(|| {
			let mut token = lexer.next();
			token.span.byte_start += byte_offset;
			token.span.byte_end += byte_offset;
			token
		})
	}

	pub fn peek(&mut self) -> &Spanned<Token> {
		match self.token.is_some() {
			true => self.token.as_ref().unwrap(),
			false => {
				self.token = Some(self.next());
				self.token.as_ref().unwrap()
			}
		}
	}
}

