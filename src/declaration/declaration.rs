use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::error::CompileError;
use crate::extension::LineOffset;
use crate::source::SourceKey;
use crate::span::Span;

pub type ModulesPending = RwLock<HashMap<Arc<ModulePath>, ModulePending>>;
pub type DeclarationsModule = RwLock<HashMap<Arc<ModulePath>, Declaration>>;
pub type DeclarationsFunction = RwLock<HashMap<Arc<FunctionPath>, Declaration>>;
pub type DeclarationsStructure = RwLock<HashMap<Arc<StructurePath>, Declaration>>;

#[derive(Debug)]
pub enum DeclarationError {
	ExpectedIdentifier,
	ExpectedModuleTerminator,
	ExpectedBlock,
	NestedExternalModule,
	UndefinedModule(Arc<ModulePath>),
	ModuleDeclarationLocation,
	ExpectedDeclaration,
	DuplicateFunction(Arc<FunctionPath>),
	DuplicateStructure(Arc<StructurePath>),
}

impl fmt::Display for DeclarationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DeclarationError::ExpectedIdentifier =>
				write!(f, "Expected an identifier"),
			DeclarationError::ExpectedModuleTerminator =>
				write!(f, "Expected line break or opening block with separator"),
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
			DeclarationError::DuplicateFunction(path) =>
				write!(f, "Function: {}, has already been declared", path),
			DeclarationError::DuplicateStructure(path) =>
				write!(f, "Structure: {}, has already been declared", path),
		}
	}
}

impl From<DeclarationError> for CompileError {
	fn from(error: DeclarationError) -> Self {
		CompileError::Declaration(error)
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ModulePath {
	pub parent: Option<Arc<ModulePath>>,
	pub identifier: Arc<str>,
}

impl ModulePath {
	pub const fn new(parent: Option<Arc<ModulePath>>, identifier: Arc<str>) -> Self {
		ModulePath { parent, identifier }
	}

	pub fn root() -> Arc<Self> {
		Arc::new(Self::new(None, "crate".into()))
	}

	pub fn append(self: Arc<ModulePath>, identifier: Arc<str>) -> Arc<Self> {
		Arc::new(ModulePath {
			parent: Some(self),
			identifier,
		})
	}
}

impl fmt::Display for ModulePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.parent.iter().try_for_each(|parent| write!(f, "{}::", parent))?;
		write!(f, "{}", self.identifier)
	}
}

#[derive(Debug, Clone)]
pub struct ModulePending {
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DeclarationPath {
	pub module_path: Arc<ModulePath>,
	pub identifier: Arc<str>,
}

impl fmt::Display for DeclarationPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}::{}", self.module_path, self.identifier)
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionPath(pub DeclarationPath);

impl fmt::Display for FunctionPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let FunctionPath(path) = self;
		write!(f, "{}", path)
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct StructurePath(pub DeclarationPath);

impl fmt::Display for StructurePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let StructurePath(path) = self;
		write!(f, "{}", path)
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
