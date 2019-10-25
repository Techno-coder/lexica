use std::sync::Arc;

use crate::error::Diagnostic;
use crate::lexer::Token;
use crate::span::{Span, Spanned};

use super::{DeclarationError, Inclusion, InclusionTerminal, ModulePath, SourceParse};

impl<'a> SourceParse<'a> {
	pub fn inclusion_root(&mut self) -> Option<()> {
		let identifier = self.context.emit(crate::parser::identifier(&mut self.lexer)
			.map_err(|diagnostic| diagnostic.note("In parsing path inclusion")))?;
		let module = ModulePath::new(None, identifier.node);
		self.context.emit(crate::parser::expect(&mut self.lexer, Token::PathSeparator))?;
		self.inclusion(module, identifier.span.byte_start)?;

		self.context.emit(crate::parser::expect(&mut self.lexer, Token::LineBreak))?;
		Some(())
	}

	fn inclusion(&mut self, mut module_path: Arc<ModulePath>, byte_start: usize) -> Option<()> {
		let mut terminal = None;
		loop {
			let token = self.lexer.next();
			match token.node {
				Token::ParenthesisOpen => return self.inclusion_list(module_path, byte_start),
				Token::Identifier(identifier) => match self.lexer.peek().node {
					Token::PathSeparator => module_path = module_path.push(identifier),
					_ => terminal = Some((InclusionTerminal::Identifier(identifier), token.span.byte_end)),
				}
				Token::Asterisk => {
					terminal = Some((InclusionTerminal::Wildcard, token.span.byte_end));
					break;
				}
				_ => {
					let error = Spanned::new(DeclarationError::ExpectedPathElement, token.span);
					return self.context.emit(Err(Diagnostic::new(error)));
				}
			}

			match self.lexer.peek().node {
				Token::PathSeparator => self.lexer.next(),
				_ => break,
			};
		}

		let (terminal, byte_end) = self.context.emit(terminal.ok_or_else(||
			Diagnostic::new(Spanned::new(DeclarationError::ExpectedPathElement,
				self.lexer.peek().span))))?;

		let span = Span::new(self.source_key, byte_start, byte_end);
		self.context.module_contexts.get_mut(&self.module_path).unwrap_or_else(||
			panic!("Module context: {}, has not been constructed", self.module_path))
			.inclusions.push(Spanned::new(Inclusion { module_path, terminal }, span));
		Some(())
	}

	fn inclusion_list(&mut self, module: Arc<ModulePath>, byte_start: usize) -> Option<()> {
		while self.lexer.peek().node != Token::ParenthesisClose {
			self.inclusion(module.clone(), byte_start)?;
			match self.lexer.peek().node {
				Token::ListSeparator => self.lexer.next(),
				_ => break,
			};
		}

		self.context.emit(crate::parser::expect(&mut self.lexer, Token::ParenthesisClose)
			.map_err(|diagnostic| diagnostic.note("In parsing inclusion list")))?;
		Some(())
	}
}
