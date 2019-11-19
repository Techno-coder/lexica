use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::{LineOffsets, StringExtension};
use crate::lexer::{Lexer, Token};
use crate::source::SourceKey;
use crate::span::{Span, Spanned};

use super::{Declaration, DeclarationError, Definition, ModuleContext, ModulePath};

#[derive(Debug)]
pub struct SourceParse<'a> {
	pub context: &'a Context,
	pub module_path: Arc<ModulePath>,
	pub physical_path: &'a Arc<PathBuf>,
	pub source_key: SourceKey,
	pub lexer: Lexer<'a>,

	line_offsets: LineOffsets,
	pub item_indents: Vec<usize>,
	pub current_module: Arc<ModulePath>,
	pub current_indent: usize,
	pub is_definition: bool,
}

impl<'a> SourceParse<'a> {
	/// Parses and loads the declarations of the source.
	pub fn parse(context: &Context, module_path: Arc<ModulePath>, declaration_span: Span,
	             physical_path: &Arc<PathBuf>, source_key: SourceKey) -> Option<()> {
		assert!(physical_path.is_file());
		let source = source_key.get(context);
		let string = context.emit(source.read_string().map_err(|error|
			Diagnostic::new(Spanned::new(error, declaration_span))))?;
		let lexer = Lexer::new(string, 0, source_key);

		SourceParse {
			context,
			module_path: module_path.clone(),
			physical_path,
			source_key,
			lexer,
			line_offsets: string.line_offsets(),
			item_indents: Vec::new(),
			current_module: module_path,
			current_indent: 0,
			is_definition: false,
		}.traverse();
		Some(())
	}

	pub fn module_context(&self) -> impl Deref<Target=ModuleContext> + DerefMut + 'a {
		use parking_lot::RwLockWriteGuard;
		RwLockWriteGuard::map(self.context.module_contexts.write(),
			|modules| modules.get_mut(&self.module_path).unwrap_or_else(||
				panic!("Module context: {}, has not been constructed", self.module_path)))
	}

	fn traverse(&mut self) {
		loop {
			let token = self.lexer.next();
			self.handle_block_change(&token);
			let (&line_offset, _) = self.line_offsets
				.range(..=token.span.byte_start).next_back().unwrap();

			match token.node {
				Token::End => break,
				Token::BlockOpen | Token::BlockClose | Token::LineBreak => continue,
				Token::Function => (),
				_ if self.is_definition => {
					let error = Spanned::new(DeclarationError::DefinitionItem, token.span);
					let _: Option<!> = self.context.emit(Err(Diagnostic::new(error)));
					self.advance_until_break();
					continue;
				}
				Token::Module | Token::Data => (),
				Token::Export => continue,
				Token::Use => {
					if self.inclusion_root().is_none() {
						self.advance_until_break();
					}
					continue;
				}
				Token::Define => {
					let declaration = Declaration { source: self.source_key, line_offset };
					self.module_context().definitions.push(Definition::new(declaration));
					self.item_indents.push(self.current_indent);
					self.advance_until_break();
					self.is_definition = true;
					continue;
				}
				_ => {
					let error = Spanned::new(DeclarationError::ExpectedDeclaration, token.span);
					let _: Option<!> = self.context.emit(Err(Diagnostic::new(error)));
					self.advance_until_break();
					continue;
				}
			}

			let identifier_token = self.lexer.next();
			let identifier = match identifier_token.node {
				Token::Identifier(identifier) => identifier,
				_ => {
					let error = Spanned::new(DeclarationError::ExpectedIdentifier, identifier_token.span);
					let _: Option<!> = self.context.emit(Err(Diagnostic::new(error)));
					self.advance_until_break();
					continue;
				}
			};

			let placement_span = token.span.extend(identifier_token.span.byte_end);
			match token.node {
				Token::Data | Token::Function => {
					let declaration = Declaration { source: self.source_key, line_offset };
					match token.node {
						Token::Data => self.structure(identifier, declaration, placement_span),
						Token::Function => self.function(identifier, declaration, placement_span),
						_ => unreachable!(),
					};

					self.skip_next_block();
					Some(())
				}
				Token::Module => {
					let declaration_span = Span::new(self.source_key,
						*line_offset, placement_span.byte_end);
					self.module(identifier, declaration_span, placement_span)
				}
				_ => unreachable!(),
			};
		}
	}
}
