use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use chashmap::CHashMap;

use crate::context::Context;
use crate::error::{CompileError, Diagnostic};
use crate::extension::LineOffset;
use crate::source::SourceKey;
use crate::span::{Span, Spanned};

use super::{FunctionPath, ModuleContext, ModulePath, StructurePath};

pub type DeclarationsFunction = CHashMap<Arc<FunctionPath>, Declaration>;
pub type DeclarationsStructure = CHashMap<Arc<StructurePath>, Declaration>;

#[derive(Debug)]
pub enum DeclarationError {
	ExpectedIdentifier,
	ExpectedConstructTerminator,
	ExpectedBlock,
	NestedExternalModule,
	UndefinedModule(Arc<ModulePath>),
	ModuleDeclarationLocation,
	ExpectedDeclaration,
	DuplicateMethod(Arc<str>),
	DuplicateFunction(Arc<FunctionPath>),
	DuplicateStructure(Arc<StructurePath>),
	ExpectedPathElement,
	DefinitionItem,
}

impl fmt::Display for DeclarationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DeclarationError::ExpectedIdentifier =>
				write!(f, "Expected an identifier"),
			DeclarationError::ExpectedConstructTerminator =>
				write!(f, "Expected line break or separator with opening block"),
			DeclarationError::ExpectedBlock =>
				write!(f, "Expected opening block"),
			DeclarationError::NestedExternalModule =>
				write!(f, "External module declarations must be at top level"),
			DeclarationError::UndefinedModule(path) =>
				write!(f, "Module definition: {}, does not exist", path),
			DeclarationError::ModuleDeclarationLocation =>
				write!(f, "Module can only be declared in root or module file"),
			DeclarationError::ExpectedDeclaration =>
				write!(f, "Expected a module, function, or structure declaration"),
			DeclarationError::DuplicateMethod(identifier) =>
				write!(f, "Method: {}, has already been declared", identifier),
			DeclarationError::DuplicateFunction(path) =>
				write!(f, "Function: {}, has already been declared", path),
			DeclarationError::DuplicateStructure(path) =>
				write!(f, "Structure: {}, has already been declared", path),
			DeclarationError::ExpectedPathElement =>
				write!(f, "Expected path element"),
			DeclarationError::DefinitionItem =>
				write!(f, "Structure definitions must only contain functions"),
		}
	}
}

impl From<DeclarationError> for CompileError {
	fn from(error: DeclarationError) -> Self {
		CompileError::Declaration(error)
	}
}

#[derive(Debug, Clone)]
pub struct ModulePending {
	pub module_path: Arc<ModulePath>,
	pub expected_path: Arc<PathBuf>,
	pub expected_module_path: Option<Arc<PathBuf>>,
	pub declaration_span: Span,
}

impl Into<Declaration> for ModulePending {
	fn into(self) -> Declaration {
		Declaration {
			source: self.declaration_span.source,
			line_offset: LineOffset(self.declaration_span.byte_start),
		}
	}
}

#[derive(Debug)]
pub struct Declaration {
	pub source: SourceKey,
	pub line_offset: LineOffset,
}

impl Declaration {
	pub fn span(&self) -> Span {
		Span::new_point(self.source, *self.line_offset)
	}
}

pub fn module_root(context: &Context, path: PathBuf) -> Option<()> {
	load_module(context, ModulePending {
		module_path: ModulePath::root(),
		expected_path: Arc::new(path),
		expected_module_path: None,
		declaration_span: Span::INTERNAL,
	})
}

/// Recursively loads all modules.
pub fn load_module(context: &Context, module: ModulePending) -> Option<()> {
	let mut sources = Vec::new();
	let mut source_errors = Vec::new();

	match crate::source::source_key(context, &module.expected_path) {
		Ok(source_key) => sources.push((source_key, &module.expected_path)),
		Err(error) => source_errors.push(error),
	}

	if let Some(expected_module_path) = &module.expected_module_path {
		match crate::source::source_key(context, &expected_module_path) {
			Ok(source_key) => sources.push((source_key, expected_module_path)),
			Err(error) => source_errors.push(error),
		}
	}

	if sources.is_empty() {
		let error = DeclarationError::UndefinedModule(module.module_path);
		let diagnostic = Diagnostic::new(Spanned::new(error, module.declaration_span));
		let diagnostic = source_errors.into_iter().fold(diagnostic, |diagnostic, error|
			diagnostic.note(error.to_string()));
		return context.emit(Err(diagnostic));
	}

	context.module_contexts.write().insert(module.module_path.clone(),
		ModuleContext::new(module.module_path.clone(), module.declaration_span)).unwrap_none();
	sources.into_iter().try_for_each(|(source_key, physical_path)|
		super::SourceParse::parse(context, module.module_path.clone(), module.declaration_span,
			physical_path, source_key))
}
