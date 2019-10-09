use std::path::PathBuf;
use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::{LineOffsets, StringExtension};
use crate::lexer::{Lexer, Token};
use crate::source::SourceKey;
use crate::span::{Span, Spanned};

use super::{Declaration, DeclarationError, DeclarationPath, FunctionPath, ModulePath,
	ModulePending, StructurePath};

pub const ROOT_FILE: &str = "main.lx";
pub const MODULE_FILE: &str = "module.lx";

#[derive(Debug)]
pub struct SourceParse<'a> {
	context: &'a Context,
	module_path: Arc<ModulePath>,
	physical_path: &'a Arc<PathBuf>,
	source_key: SourceKey,
	lexer: Lexer<'a>,

	line_offsets: LineOffsets,
	module_indents: Vec<usize>,
	current_module: Arc<ModulePath>,
	current_indent: usize,
}

impl<'a> SourceParse<'a> {
	/// Parses and loads the declarations of the source.
	pub fn parse(context: &Context, module_path: Arc<ModulePath>, declaration_span: Span,
	             physical_path: &Arc<PathBuf>, source_key: SourceKey) -> Option<()> {
		assert!(physical_path.is_file());
		let source = source_key.get(context);
		let string = context.emit(source.read_string()
			.map_err(|error| Diagnostic::new(Spanned::new(error, declaration_span))))?;
		let lexer = Lexer::new(string, 0, source_key);

		SourceParse {
			context,
			module_path: module_path.clone(),
			physical_path,
			source_key,
			lexer,
			line_offsets: string.line_offsets(),
			module_indents: Vec::new(),
			current_module: module_path,
			current_indent: 0,
		}.traverse()
	}

	fn traverse(&mut self) -> Option<()> {
		loop {
			let token = self.lexer.next();
			self.handle_block_change(&token);
			match token.node {
				Token::Module | Token::Function | Token::Data => (),
				Token::BlockOpen | Token::BlockClose => continue,
				Token::Export | Token::LineBreak => continue,
				Token::End => break Some(()),
				_ => {
					let error = Spanned::new(DeclarationError::ExpectedDeclaration, token.span);
					return self.context.emit(Err(Diagnostic::new(error)));
				}
			}

			let identifier_token = self.lexer.next();
			let identifier = match identifier_token.node {
				Token::Identifier(identifier) => identifier,
				_ => {
					let error = Spanned::new(DeclarationError::ExpectedIdentifier, identifier_token.span);
					return self.context.emit(Err(Diagnostic::new(error)
						.note(format!("In parsing declaration for: {:?}", token.node))));
				}
			};

			let (&line_offset, _) = self.line_offsets.range(..=token.span.byte_start)
				.next_back().unwrap();
			let placement_span = token.span.extend(identifier_token.span.byte_end);
			match token.node {
				Token::Data | Token::Function => {
					let module_path = self.current_module.clone();
					let path = DeclarationPath { module_path, identifier };
					let declaration = Declaration { source: self.source_key, line_offset };

					match token.node {
						Token::Data => self.structure(Arc::new(StructurePath(path)),
							declaration, placement_span),
						Token::Function => self.function(Arc::new(FunctionPath(path)),
							declaration, placement_span),
						_ => unreachable!(),
					};

					self.skip_next_block();
				}
				Token::Module => {
					let declaration_span = Span::new(self.source_key,
						*line_offset, placement_span.byte_end);
					self.module(identifier, declaration_span, placement_span)?;
				}
				_ => unreachable!(),
			}
		}
	}

	fn structure(&mut self, structure_path: Arc<StructurePath>, declaration: Declaration, placement_span: Span) {
		let mut declarations = self.context.declarations_structure.write();
		match declarations.get(&structure_path) {
			None => declarations.insert(structure_path, declaration),
			Some(declaration) => {
				let (location, _) = declaration.span().location(self.context);
				let error = DeclarationError::DuplicateStructure(structure_path);
				self.context.emit(Err(Diagnostic::new(Spanned::new(error, placement_span))
					.note(format!("Duplicate declared in: {}", location))))
			}
		};
	}

	fn function(&mut self, function_path: Arc<FunctionPath>, declaration: Declaration, placement_span: Span) {
		let mut declarations = self.context.declarations_function.write();
		match declarations.get(&function_path) {
			None => declarations.insert(function_path, declaration),
			Some(declaration) => {
				let (location, _) = declaration.span().location(self.context);
				let error = DeclarationError::DuplicateFunction(function_path);
				self.context.emit(Err(Diagnostic::new(Spanned::new(error, placement_span))
					.note(format!("Duplicate declared in: {}", location))))
			}
		};
	}

	fn module(&mut self, identifier: Arc<str>, declaration_span: Span, placement_span: Span) -> Option<()> {
		match self.lexer.next().node {
			Token::Separator => {
				self.require_block();
				self.module_indents.push(self.current_indent);
				self.current_module = self.current_module.clone().append(identifier);
				Some(())
			}
			Token::LineBreak => {
				if self.current_module != self.module_path {
					let error = Spanned::new(DeclarationError::NestedExternalModule, placement_span);
					return self.context.emit(Err(Diagnostic::new(error)));
				}

				let file_name = self.physical_path.file_name().unwrap();
				if file_name != ROOT_FILE && file_name != MODULE_FILE {
					let error = Spanned::new(DeclarationError::ModuleDeclarationLocation, placement_span);
					return self.context.emit(Err(Diagnostic::new(error)));
				}

				let parent = self.physical_path.parent().unwrap();
				let expected_path = parent.join(format!("{}.lx", identifier));
				let expected_module_path = parent.join(format!("{}/module.lx", identifier));

				let module = ModulePending {
					expected_path: Arc::new(expected_path),
					expected_module_path: Some(Arc::new(expected_module_path)),
					declaration_span,
				};

				let module_path = self.current_module.clone().append(identifier);
				self.context.modules_pending.write().insert(module_path, module);
				Some(())
			}
			_ => {
				let error = Spanned::new(DeclarationError::ExpectedModuleTerminator, placement_span);
				self.context.emit(Err(Diagnostic::new(error)))
			}
		}
	}

	fn handle_block_change(&mut self, token: &Spanned<Token>) {
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

	fn skip_next_block(&mut self) {
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

	fn advance_until_break(&mut self) {
		loop {
			match self.lexer.peek().node {
				Token::BlockOpen => break,
				Token::LineBreak => break,
				_ => self.lexer.next(),
			};
		}
	}

	fn require_block(&mut self) {
		match self.lexer.peek().node {
			Token::BlockOpen => (),
			_ => {
				let error = Spanned::new(DeclarationError::ExpectedBlock, self.lexer.peek().span);
				let _: Option<()> = self.context.emit(Err(Diagnostic::new(error)));
			}
		}
	}
}
