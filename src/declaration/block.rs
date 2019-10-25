use crate::error::Diagnostic;
use crate::lexer::Token;
use crate::span::Spanned;

use super::{DeclarationError, SourceParse};

impl<'a> SourceParse<'a> {
	/// Adjusts the indentation level if the token is a block change.
	pub fn handle_block_change(&mut self, token: &Spanned<Token>) {
		match token.node {
			Token::BlockOpen => self.current_indent += 1,
			Token::BlockClose => {
				self.current_indent -= 1;
				if let Some(indent) = self.module_indents.last() {
					if &self.current_indent == indent {
						self.module_indents.pop().unwrap();
						self.current_module = self.current_module.parent.as_ref()
							.unwrap().clone();
					}
				}
			}
			_ => (),
		}
	}

	/// Consumes the block after the next break.
	pub fn skip_next_block(&mut self) {
		self.advance_until_break();
		if self.lexer.peek().node == Token::LineBreak {
			self.lexer.next();
			return;
		}

		self.require_block();
		let target_indent = self.current_indent;
		loop {
			let token = self.lexer.next();
			match token.node {
				Token::BlockOpen => self.current_indent += 1,
				Token::BlockClose => self.current_indent -= 1,
				Token::End => break,
				_ => (),
			}

			if self.current_indent == target_indent {
				break;
			}
		}
	}

	/// Consumes a BlockOpen or otherwise emits an error.
	pub fn require_block(&mut self) {
		match self.lexer.peek().node {
			Token::BlockOpen => (),
			_ => {
				let error = Spanned::new(DeclarationError::ExpectedBlock, self.lexer.peek().span);
				let _: Option<()> = self.context.emit(Err(Diagnostic::new(error)));
			}
		}
	}

	/// Skips tokens until a BlockOpen or LineBreak.
	pub fn advance_until_break(&mut self) {
		loop {
			match self.lexer.peek().node {
				Token::BlockOpen => break,
				Token::LineBreak => break,
				_ => self.lexer.next(),
			};
		}
	}
}
