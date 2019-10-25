use std::sync::Arc;

use crate::error::Diagnostic;
use crate::lexer::Token;
use crate::span::{Span, Spanned};

use super::{Declaration, DeclarationError, FunctionPath, ModuleContext,
	ModulePending, SourceParse, StructurePath};

const ROOT_FILE: &str = "main.lx";
const MODULE_FILE: &str = "module.lx";

impl<'a> SourceParse<'a> {
	pub fn structure(&mut self, structure_path: Arc<StructurePath>, declaration: Declaration, placement_span: Span) {
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

	pub fn function(&mut self, function_path: Arc<FunctionPath>, declaration: Declaration, placement_span: Span) {
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

	pub fn module(&mut self, identifier: Arc<str>, declaration_span: Span, placement_span: Span) -> Option<()> {
		match self.lexer.next().node {
			Token::Separator => self.nested(identifier),
			Token::LineBreak => self.external(identifier, declaration_span, placement_span),
			_ => {
				let error = Spanned::new(DeclarationError::ExpectedConstructTerminator, placement_span);
				self.context.emit(Err(Diagnostic::new(error)))
			}
		}
	}

	fn nested(&mut self, identifier: Arc<str>) -> Option<()> {
		self.require_block();
		self.module_indents.push(self.current_indent);
		self.current_module = self.current_module.clone().append(identifier);
		self.context.module_contexts.write()
			.insert(self.current_module.clone(), ModuleContext::default());
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

		let module = ModulePending {
			expected_path: Arc::new(expected_path),
			expected_module_path: Some(Arc::new(expected_module_path)),
			declaration_span,
		};

		let module_path = self.current_module.clone().append(identifier);
		self.context.modules_pending.write().insert(module_path, module);
		Some(())
	}
}
