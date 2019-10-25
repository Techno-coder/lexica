use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ModulePath {
	pub parent: Option<Arc<ModulePath>>,
	pub identifier: Arc<str>,
}

impl ModulePath {
	pub fn new(parent: Option<Arc<ModulePath>>, identifier: Arc<str>) -> Arc<Self> {
		Arc::new(ModulePath { parent, identifier })
	}

	pub fn unresolved() -> Arc<Self> {
		Self::new(None, "?".into())
	}

	pub fn is_unresolved(&self) -> bool {
		self.identifier.as_ref() == "?"
	}

	pub fn intrinsic() -> Arc<Self> {
		Self::new(None, "intrinsic".into())
	}

	pub fn root() -> Arc<Self> {
		Self::new(None, "crate".into())
	}

	pub fn push(self: Arc<ModulePath>, identifier: Arc<str>) -> Arc<Self> {
		Self::new(Some(self), identifier)
	}
}

impl fmt::Display for ModulePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.parent.iter().try_for_each(|parent| write!(f, "{}::", parent))?;
		write!(f, "{}", self.identifier)
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
