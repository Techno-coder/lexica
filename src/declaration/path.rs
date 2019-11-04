use std::fmt;
use std::sync::Arc;

#[derive(Clone, Hash, Eq, PartialEq)]
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

	pub fn any_unresolved(&self) -> bool {
		self.is_unresolved() || self.parent.as_ref().map(|parent|
			parent.any_unresolved()).unwrap_or(false)
	}

	pub fn intrinsic() -> Arc<Self> {
		Self::new(None, "intrinsic".into())
	}

	pub fn root() -> Arc<Self> {
		Self::new(None, "crate".into())
	}

	pub fn is_root(&self) -> bool {
		self.identifier.as_ref() == "crate"
	}

	/// Finds the last resolved path element.
	pub fn head(&self) -> Option<Arc<ModulePath>> {
		match self.is_unresolved() {
			false => self.parent.as_ref().and_then(|parent| parent.head())
				.or_else(|| Some(Arc::new(self.clone()))),
			true => None,
		}
	}

	/// Removes the head element.
	pub fn tail(self: Arc<ModulePath>) -> Arc<Self> {
		match &self.parent {
			Some(parent) if parent.parent.is_some() =>
				parent.clone().tail().push(self.identifier.clone()),
			_ => ModulePath::new(None, self.identifier.clone())
		}
	}

	pub fn push(self: Arc<ModulePath>, identifier: Arc<str>) -> Arc<Self> {
		Self::new(Some(self), identifier)
	}

	/// Appends another module path. Ignores unresolved path elements.
	pub fn append(mut self: Arc<ModulePath>, other: &Arc<ModulePath>) -> Arc<Self> {
		if let Some(parent) = &other.parent {
			self = self.append(parent);
		}

		match other.is_unresolved() {
			false => self.push(other.identifier.clone()),
			true => self,
		}
	}
}

impl fmt::Display for ModulePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.parent.iter().try_for_each(|parent| write!(f, "{}::", parent))?;
		write!(f, "{}", self.identifier)
	}
}

impl fmt::Debug for ModulePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct DeclarationPath {
	pub module_path: Arc<ModulePath>,
	pub identifier: Arc<str>,
}

impl DeclarationPath {
	pub fn head(&self) -> Arc<str> {
		self.module_path.head().map(|module_path| module_path.identifier.clone())
			.unwrap_or_else(|| self.identifier.clone())
	}
}

impl fmt::Display for DeclarationPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}::{}", self.module_path, self.identifier)
	}
}

impl fmt::Debug for DeclarationPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct FunctionPath(pub DeclarationPath);

impl fmt::Display for FunctionPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let FunctionPath(path) = self;
		write!(f, "{}", path)
	}
}

impl fmt::Debug for FunctionPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "FunctionPath({})", self)
	}
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct StructurePath(pub DeclarationPath);

impl fmt::Display for StructurePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let StructurePath(path) = self;
		write!(f, "{}", path)
	}
}

impl fmt::Debug for StructurePath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "StructurePath({})", self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_module_append() {
		let module = ModulePath::root().push("first".into()).push("second".into());
		let other = &ModulePath::unresolved().push("third".into()).push("fourth".into());
		assert_eq!(module.clone().append(other), module.push("third".into()).push("fourth".into()));
	}

	#[test]
	fn test_module_tail() {
		let module = ModulePath::unresolved().push("crate".into()).push("element".into());
		assert_eq!(module.tail(), ModulePath::root().push("element".into()));
	}
}
