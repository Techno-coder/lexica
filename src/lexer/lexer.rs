use crate::source::SourceKey;
use crate::span::Spanned;

use super::direct_lexer::DirectLexer;
use super::Token;

/// Adds one token lookahead to `DirectLexer`.
/// Replaces `LineBreak` followed by `BlockOpen` with just `BlockOpen`.
#[derive(Debug)]
pub struct Lexer<'a> {
	lexer: DirectLexer<'a>,
	token: Option<Spanned<Token>>,
}

impl<'a> Lexer<'a> {
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		Lexer {
			lexer: DirectLexer::new(string, source_key),
			token: None,
		}
	}

	pub fn next(&mut self) -> Spanned<Token> {
		let lexer = &mut self.lexer;
		let token = self.token.take().unwrap_or_else(|| lexer.next());
		match token.node {
			Token::LineBreak => {
				let other = lexer.next();
				match other.node {
					Token::BlockOpen => other,
					_ => {
						self.token = Some(other);
						token
					}
				}
			}
			_ => token
		}
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

