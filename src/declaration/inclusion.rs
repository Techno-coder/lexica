use std::sync::Arc;

use crate::error::Diagnostic;
use crate::lexer::Token;
use crate::span::Spanned;

use super::{DeclarationError, Inclusion, InclusionTerminal, ModulePath, SourceParse};

impl<'a> SourceParse<'a> {
	pub fn inclusion_root(&mut self) -> Option<()> {
		let module = self.context.emit(crate::parser::identifier(&mut self.lexer)
			.map_err(|diagnostic| diagnostic.note("In parsing path inclusion")))
			.map(|identifier| Arc::new(ModulePath::new(None, identifier.node)))?;
		self.context.emit(crate::parser::expect(&mut self.lexer, Token::PathSeparator))?;
		self.inclusion(module)?;

		self.context.emit(crate::parser::expect(&mut self.lexer, Token::LineBreak))?;
		Some(())
	}

	fn inclusion(&mut self, mut module: Arc<ModulePath>) -> Option<()> {
		let mut terminal = None;
		loop {
			let token = self.lexer.next();
			match token.node {
				Token::ParenthesisOpen => return self.inclusion_list(module),
				Token::Identifier(identifier) => match self.lexer.peek().node {
					Token::PathSeparator => module = module.append(identifier),
					_ => terminal = Some(InclusionTerminal::Identifier(identifier)),
				}
				Token::Asterisk => {
					terminal = Some(InclusionTerminal::Wildcard);
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

		let terminal = self.context.emit(terminal.ok_or_else(||
			Diagnostic::new(Spanned::new(DeclarationError::ExpectedPathElement,
				self.lexer.peek().span))))?;
		self.context.module_contexts.write().get_mut(&self.module_path).unwrap_or_else(||
			panic!("Module context: {}, has not been constructed", self.module_path))
			.inclusions.push(Inclusion { module, terminal });
		Some(())
	}

	fn inclusion_list(&mut self, module: Arc<ModulePath>) -> Option<()> {
		while self.lexer.peek().node != Token::ParenthesisClose {
			self.inclusion(module.clone())?;
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
