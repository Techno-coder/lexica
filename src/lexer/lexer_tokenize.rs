use std::iter::Peekable;

use crate::source::SourceKey;
use crate::span::Spanned;

use super::source_split::SourceSplit;
use super::token::{LexerToken, Token};

/// Parses a string into lexer tokens.
/// Annotates string slices provided by `SourceSplit`.
#[derive(Debug, Clone)]
pub struct LexerTokenize<'a> {
	source: Peekable<SourceSplit<'a>>,
	was_whitespace: bool,
}

impl<'a> LexerTokenize<'a> {
	pub fn new(string: &'a str, source_key: SourceKey) -> Self {
		let split = SourceSplit::new(string, source_key);
		LexerTokenize { source: split.peekable(), was_whitespace: true }
	}
}

impl<'a> Iterator for LexerTokenize<'a> {
	type Item = Spanned<LexerToken>;

	fn next(&mut self) -> Option<Self::Item> {
		let lexeme = self.source.next()?;
		let is_whitespace = whitespace(Some(&lexeme));
		let next_whitespace = whitespace(self.source.peek());
		let whitespace = self.was_whitespace && next_whitespace;
		self.was_whitespace = is_whitespace;

		Some(Spanned::new(match lexeme.node {
			"\t" => LexerToken::Indent,
			token => LexerToken::Token(match token {
				"\n" => Token::LineBreak,
				"fn" => Token::Function,
				"data" => Token::Data,
				"define" => Token::Define,
				"module" => Token::Module,
				"export" => Token::Export,
				"use" => Token::Use,
				"let" => Token::Let,
				"loop" => Token::Loop,
				"drop" => Token::Drop,
				"if" => Token::If,
				"self" => Token::SelfVariable,
				"&" => Token::Reference,
				"~&" => Token::Unique,
				"#" => Token::Compile,
				"_" => Token::Wildcard,
				"(" => Token::ParenthesisOpen,
				")" => Token::ParenthesisClose,
				"." => Token::Dot,
				":" => Token::Separator,
				"::" => Token::PathSeparator,
				"," => Token::ListSeparator,
				"$" => Token::Template,
				"~" => Token::Mutable,
				"'" => Token::Prime,
				"+" => Token::Add,
				"-" => Token::Minus,
				"*" => match whitespace {
					false => Token::Asterisk,
					true => Token::Multiply,
				},
				"+=" => Token::AddAssign,
				"-=" => Token::MinusAssign,
				"*=" => Token::MultiplyAssign,
				"=" => Token::Assign,
				"<" => Token::AngleLeft,
				">" => Token::AngleRight,
				"<=" => Token::LessEqual,
				">=" => Token::GreaterEqual,
				"==" => Token::Equality,
				"=>" => Token::Implies,
				"<=>" => Token::Swap,
				"->" => Token::ReturnSeparator,
				_ if is_whitespace => return self.next(),
				other => {
					if let Ok(integer) = other.parse::<i128>() {
						Token::Integer(integer)
					} else if let Ok(truth) = other.parse::<bool>() {
						Token::Truth(truth)
					} else {
						Token::Identifier(other.into())
					}
				}
			})
		}, lexeme.span))
	}
}

fn whitespace(lexeme: Option<&Spanned<&str>>) -> bool {
	lexeme.and_then(|lexeme| lexeme.node.chars().next().map(char::is_whitespace)) == Some(true)
}

#[cfg(test)]
mod tests {
	use LexerToken::*;
	use LexerToken::Token;

	use crate::span::Span;

	use super::*;
	use super::Token::*;

	#[test]
	fn test_declaration() {
		let string = "\tfn identifier(argument: type):";
		let lexemes: Vec<_> = LexerTokenize::new(string, SourceKey::INTERNAL)
			.map(|lexeme| lexeme.node).collect();
		assert_eq!(lexemes, &[Indent, Token(Function), Token(Identifier("identifier".into())),
			Token(ParenthesisOpen), Token(Identifier("argument".into())), Token(Separator),
			Token(Identifier("type".into())), Token(ParenthesisClose), Token(Separator)]);
	}

	#[test]
	fn test_whitespace() {
		let string = "use test::*\n*(2 * 3)";
		let lexemes: Vec<_> = LexerTokenize::new(string, SourceKey::INTERNAL)
			.map(|lexeme| lexeme.node).collect();
		assert_eq!(lexemes, &[Token(Use), Token(Identifier("test".into())), Token(PathSeparator),
			Token(Asterisk), Token(LineBreak), Token(Asterisk), Token(ParenthesisOpen),
			Token(Integer(2)), Token(Multiply), Token(Integer(3)), Token(ParenthesisClose)]);
	}

	#[test]
	fn test_whitespace_kind() {
		assert_eq!(whitespace(Some(&Spanned::new("\t\t\t\t", Span::INTERNAL))), true);
		assert_eq!(whitespace(Some(&Spanned::new("identifier", Span::INTERNAL))), false);
		assert_eq!(whitespace(Some(&Spanned::new("*", Span::INTERNAL))), false);
	}
}
