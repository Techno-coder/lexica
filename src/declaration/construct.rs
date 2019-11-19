use std::sync::Arc;

use crate::error::Diagnostic;
use crate::lexer::Token;
use crate::span::{Span, Spanned};

use super::{Declaration, DeclarationError, DeclarationPath, FunctionPath,
	ModuleContext, ModulePending, SourceParse, StructurePath};

const ROOT_FILE: &str = "main.lx";
const MODULE_FILE: &str = "module.lx";

impl<'a> SourceParse<'a> {
	pub fn structure(&mut self, identifier: Arc<str>, declaration: Declaration, placement_span: Span) {
		let module_path = self.current_module.clone();
		let path = DeclarationPath { module_path, identifier };
		let structure_path = Arc::new(StructurePath(path));
		let declarations = &self.context.declarations_structure;
		match declarations.get(&structure_path) {
			None => declarations.insert(structure_path, declaration),
			Some(declaration) => {
				let location = declaration.span().location(self.context);
				let error = DeclarationError::DuplicateStructure(structure_path);
				self.context.emit(Err(Diagnostic::new(Spanned::new(error, placement_span))
					.note(format!("Duplicate declared in: {}", location))))
			}
		};
	}

	pub fn function(&mut self, identifier: Arc<str>, declaration: Declaration, placement_span: Span) {
		let declarations = &self.context.declarations_function;
		let (error, duplicate) = match self.is_definition {
			false => {
				let module_path = self.current_module.clone();
				let path = DeclarationPath { module_path, identifier };
				let function_path = Arc::new(FunctionPath(path));
				match declarations.get(&function_path) {
					None => return declarations.insert(function_path, declaration).unwrap_none(),
					Some(duplicate) => (DeclarationError::DuplicateFunction(function_path), duplicate.span()),
				}
			}
			true => {
				let definitions = &mut self.module_context().definitions;
				let definition = definitions.last_mut().unwrap();
				match definition.methods.get(&identifier) {
					None => return definition.methods.insert(identifier, declaration).unwrap_none(),
					Some(duplicate) => (DeclarationError::DuplicateMethod(identifier), duplicate.span()),
				}
			}
		};

		let error = Diagnostic::new(Spanned::new(error, placement_span))
			.note(format!("Duplicate declared in: {}", duplicate.location(self.context)));
		let _: Option<!> = self.context.emit(Err(error));
	}

	pub fn module(&mut self, identifier: Arc<str>, declaration_span: Span, placement_span: Span) -> Option<()> {
		match self.lexer.next().node {
			Token::Separator => self.nested(identifier, placement_span),
			Token::LineBreak => self.external(identifier, declaration_span, placement_span),
			_ => {
				let error = Spanned::new(DeclarationError::ExpectedConstructTerminator, placement_span);
				self.context.emit(Err(Diagnostic::new(error)))
			}
		}
	}

	fn nested(&mut self, identifier: Arc<str>, placement_span: Span) -> Option<()> {
		self.require_block();
		self.item_indents.push(self.current_indent);
		self.current_module = self.current_module.clone().push(identifier);
		let module_context = ModuleContext::new(self.current_module.clone(), placement_span);
		self.context.module_contexts.write().insert(self.current_module.clone(), module_context);
		Some(())
	}

	fn external(&mut self, identifier: Arc<str>, declaration_span: Span, placement_span: Span) -> Option<()> {
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
		super::load_module(self.context, ModulePending {
			module_path: self.current_module.clone().push(identifier),
			expected_path: Arc::new(expected_path),
			expected_module_path: Some(Arc::new(expected_module_path)),
			declaration_span,
		})
	}
}
