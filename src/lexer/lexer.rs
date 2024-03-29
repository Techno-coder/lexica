use crate::declaration::Declaration;
use crate::error::Diagnostic;
use crate::source::{Source, SourceKey};
use crate::span::Spanned;

use super::space_lexer::SpaceLexer;
use super::Token;

/// Adds one token lookahead.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
	lexer: SpaceLexer<'a>,
	token: Option<Spanned<Token>>,
	byte_offset: usize,
}

impl<'a> Lexer<'a> {
	/// Creates a new `Lexer` instance.
	/// `byte_offset` specifies where to start lexing.
	pub fn new(string: &'a str, byte_offset: usize, source_key: SourceKey) -> Self {
		Lexer {
			lexer: SpaceLexer::new(&string[byte_offset..], source_key),
			token: None,
			byte_offset,
		}
	}

	pub fn declaration(source: &'a Source, declaration: &Declaration) -> Result<Self, Diagnostic> {
		let string = source.read_string().map_err(|error|
			Diagnostic::new(Spanned::new(error, declaration.span())))?;
		Ok(Self::new(string, *declaration.line_offset, declaration.source))
	}

	/// Ignores the next token and returns itself.
	pub fn consume(&mut self) -> &mut Self {
		self.next();
		self
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

