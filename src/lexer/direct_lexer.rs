use std::cmp::Ordering;
use std::iter::Peekable;

use crate::source::SourceKey;
use crate::span::{Span, Spanned};

use super::lexer_tokenize::LexerTokenize;
use super::token::{LexerToken, Token};

type LexerTokenizer<'a> = Peekable<LexerTokenize<'a>>;

/// Flattens lexer tokens into parser tokens.
/// Resolves indentation into semantic blocks.
/// Emits `End` tokens upon stream exhaustion.
#[derive(Debug)]
pub struct DirectLexer<'a> {
	lexer: LexerTokenizer<'a>,
	end_token: Spanned<LexerToken>,

	base_indent: usize,
	indent_level: usize,
	current_indent: usize,
	current_indent_span: Option<Span>,
	is_new_line: bool,
}

impl<'a> DirectLexer<'a> {
	// TODO: Accept string offset
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		let mut lexer = LexerTokenize::new(string, source_key).peekable();
		let end_span = Span::new(source_key, string.len(), string.len() + 1);
		let end_token = Spanned::new(LexerToken::Token(Token::End), end_span);

		let mut base_indent = 0;
		while let Some(Spanned { node: LexerToken::Indent, .. }) = lexer.peek() {
			base_indent += 1;
			lexer.next();
		}

		DirectLexer {
			lexer,
			end_token,
			base_indent,
			indent_level: base_indent,
			current_indent: base_indent,
			current_indent_span: None,
			is_new_line: false,
		}
	}

	pub fn next(&mut self) -> Spanned<Token> {
		if let Some(token) = self.resolve_indent() {
			return token;
		}

		if self.is_new_line {
			self.is_new_line = false;
			let start_span = self.lexer.peek().unwrap_or(&self.end_token).span;
			let mut byte_end = start_span.byte_end;
			let mut indent_level = 0;

			while let Some(token) = self.lexer.peek() {
				match token.node {
					LexerToken::Indent => {
						indent_level += 1;
						byte_end = token.span.byte_end;
						self.lexer.next();
					}
					LexerToken::Token(Token::LineBreak) => return self.next(),
					_ => break,
				}
			}

			self.current_indent = indent_level;
			self.current_indent_span = Some(start_span.extend(byte_end));
			return self.next();
		}

		let lexer_token = self.lexer.next().unwrap_or(self.end_token.clone());
		match lexer_token.node {
			LexerToken::Token(token) => match token {
				Token::LineBreak => {
					self.is_new_line = true;
					return Spanned::new(token, lexer_token.span);
				}
				Token::End if self.indent_level != 0 => {
					self.current_indent = 0;
					self.current_indent_span = Some(lexer_token.span);
				}
				_ => return Spanned::new(token, lexer_token.span),
			}
			LexerToken::Indent => (),
		}

		self.next()
	}

	fn resolve_indent(&mut self) -> Option<Spanned<Token>> {
		let token = match usize::cmp(&self.indent_level, &self.current_indent) {
			Ordering::Less => {
				self.indent_level += 1;
				Token::BlockOpen
			}
			Ordering::Greater => {
				self.indent_level -= 1;
				Token::BlockClose
			}
			Ordering::Equal => {
				self.current_indent_span = None;
				return None;
			}
		};

		let span = self.current_indent_span.as_ref().unwrap();
		Some(Spanned::new(token, *span))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_indentation() {
		let tokens = collect(DirectLexer::new("\t\t\n\t\t\t\t:\n\t\t", SourceKey::INTERNAL));
		assert_eq!(&tokens, &[Token::LineBreak, Token::BlockOpen, Token::BlockOpen,
			Token::Separator, Token::LineBreak, Token::BlockClose, Token::BlockClose,
			Token::BlockClose, Token::BlockClose, Token::End]);
	}

	#[test]
	fn test_blank_line() {
		let tokens = collect(DirectLexer::new("\t\n\n\t\t:\n", SourceKey::INTERNAL));
		assert_eq!(&tokens, &[Token::LineBreak, Token::LineBreak, Token::BlockOpen,
			Token::Separator, Token::LineBreak, Token::BlockClose, Token::BlockClose, Token::End]);
	}

	fn collect(mut lexer: DirectLexer) -> Vec<Token> {
		let mut tokens = Vec::new();
		loop {
			let token = lexer.next();
			match token.node {
				Token::End => {
					tokens.push(token.node);
					return tokens;
				}
				_ => tokens.push(token.node),
			}
		}
	}
}
