use crate::source::SourceKey;
use crate::span::Spanned;

use super::source_split::SourceSplit;
use super::token::{LexerToken, Token};

/// Parses a string into lexer tokens.
/// Annotates string slices provided by `SourceSplit`.
#[derive(Debug)]
pub struct LexerTokenize<'a> {
	source: SourceSplit<'a>,
}

impl<'a> LexerTokenize<'a> {
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		LexerTokenize { source: SourceSplit::new(string, source_key) }
	}
}

impl<'a> Iterator for LexerTokenize<'a> {
	type Item = Spanned<LexerToken>;

	fn next(&mut self) -> Option<Self::Item> {
		let lexeme = self.source.next()?;
		Some(Spanned::new(match lexeme.node {
			"\t" => LexerToken::Indent,
			token => LexerToken::Token(match token {
				"\n" => Token::LineBreak,
				"fn" => Token::Function,
				"data" => Token::Data,
				"module" => Token::Module,
				"export" => Token::Export,
				"let" => Token::Let,
				"loop" => Token::Loop,
				"if" => Token::If,
				"_" => Token::Wildcard,
				"(" => Token::ParenthesisOpen,
				")" => Token::ParenthesisClose,
				":" => Token::Separator,
				"," => Token::ListSeparator,
				"~" => Token::Mutable,
				"=" => Token::Equals,
				"=>" => Token::Implies,
				"<=>" => Token::Swap,
				"->" => Token::ReturnSeparator,
				other => Token::Identifier(other.into()),
			})
		}, lexeme.span))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_declaration() {
		let string = "\tfn identifier(argument: type):";
		let lexemes: Vec<_> = LexerTokenize::new(string, SourceKey::INTERNAL)
			.filter_map(|lexeme| match lexeme.node {
				LexerToken::Token(Token::Identifier(_)) => None,
				LexerToken::Token(token) => Some(token),
				_ => None
			}).collect();
		assert_eq!(&lexemes, &[Token::Function, Token::ParenthesisOpen, Token::Separator,
			Token::ParenthesisClose, Token::Separator]);
	}
}
