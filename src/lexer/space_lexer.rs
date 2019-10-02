use crate::lexer::Token;
use crate::source::SourceKey;
use crate::span::Spanned;

use super::indent_lexer::IndentLexer;

/// Replaces `LineBreak` followed by `BlockOpen` with `BlockOpen`.
/// Ignores `LineBreak` and block changes within brackets.
#[derive(Debug)]
pub struct SpaceLexer<'a> {
	lexer: IndentLexer<'a>,
	brackets: Vec<Token>,
	bracket_indent: usize,
	buffer: Option<Spanned<Token>>,
}

impl<'a> SpaceLexer<'a> {
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		SpaceLexer {
			lexer: IndentLexer::new(string, source_key),
			brackets: Vec::new(),
			bracket_indent: 0,
			buffer: None,
		}
	}

	pub fn next(&mut self) -> Spanned<Token> {
		let token = self.next_token();
		match token.node {
			Token::ParenthesisOpen => self.brackets.push(token.node.clone()),
			Token::ParenthesisClose => self.pop_expected(Token::ParenthesisOpen),
			Token::LineBreak if !self.brackets.is_empty() => return self.next(),
			Token::BlockOpen if !self.brackets.is_empty() => return self.block_open(),
			Token::BlockClose if !self.brackets.is_empty() || self.bracket_indent > 0 =>
				return self.block_close(),
			Token::Module | Token::Function | Token::Data => self.reset(),
			_ => (),
		}
		token
	}

	fn next_token(&mut self) -> Spanned<Token> {
		let token = self.buffer.take()
			.unwrap_or_else(|| self.lexer.next());
		if let Token::LineBreak = token.node {
			let other = self.lexer.next();
			match other.node {
				Token::BlockOpen => return other,
				_ => self.buffer = Some(other),
			}
		}
		token
	}

	fn block_open(&mut self) -> Spanned<Token> {
		self.bracket_indent += 1;
		self.next()
	}

	fn block_close(&mut self) -> Spanned<Token> {
		self.bracket_indent = self.bracket_indent.saturating_sub(1);
		self.next()
	}

	fn pop_expected(&mut self, expected: Token) {
		if let Some(token) = self.brackets.last() {
			if token == &expected {
				self.brackets.pop();
			}
		}
	}

	fn reset(&mut self) {
		self.brackets.clear();
		self.bracket_indent = 0;
	}
}
