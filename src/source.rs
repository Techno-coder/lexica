use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::context::Context;
use crate::error::CompileError;

pub type SourceMap = RwLock<HashMap<SourceKey, Arc<Source>>>;
pub type SourceKeyMap = RwLock<HashMap<Arc<PathBuf>, SourceKey>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct SourceKey(usize);

impl SourceKey {
	pub const INTERNAL: Self = SourceKey(usize::max_value());

	pub fn get(&self, context: &Context) -> Arc<Source> {
		context.sources.read().get(self).cloned()
			.unwrap_or_else(|| panic!("Source key: {:?}, is not present in context", self))
	}
}

#[derive(Debug)]
pub struct Source {
	pub path: Arc<PathBuf>,
	pub data: Vec<u8>,
}

impl Source {
	pub fn read_string(&self) -> Result<&str, SourceError> {
		str::from_utf8(&self.data).map_err(|error| SourceError::InvalidString(self.path.clone(), error))
	}
}

#[derive(Debug)]
pub enum SourceError {
	MissingFile(Arc<PathBuf>),
	ReadFailure(Arc<PathBuf>, std::io::Error),
	InvalidString(Arc<PathBuf>, std::str::Utf8Error),
}

impl fmt::Display for SourceError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			SourceError::MissingFile(path) =>
				write!(f, "File: {}, does not exist", path.display()),
			SourceError::ReadFailure(path, error) =>
				write!(f, "File at: {}, could not be opened: {}", path.display(), error),
			SourceError::InvalidString(path, error) =>
				write!(f, "File at: {}, could not be read as a string: {}", path.display(), error),
		}
	}
}

impl From<SourceError> for CompileError {
	fn from(error: SourceError) -> Self {
		CompileError::Source(error)
	}
}

pub fn source_key(context: &Context, path: &Arc<PathBuf>) -> Result<SourceKey, SourceError> {
	if let Some(key) = context.source_keys.read().get(path) {
		return Ok(*key);
	}

	let mut data = Vec::new();
	let mut file = File::open(path.deref()).map_err(|_| SourceError::MissingFile(path.clone()))?;
	file.read_to_end(&mut data).map_err(|error| SourceError::ReadFailure(path.clone(), error))?;

	let mut sources = context.sources.write();
	let mut source_keys = context.source_keys.write();
	let source_key = SourceKey(source_keys.len());
	source_keys.insert(path.clone(), source_key);

	let source = Arc::new(Source { path: path.clone(), data });
	sources.insert(source_key, source);
	Ok(source_key)
}
