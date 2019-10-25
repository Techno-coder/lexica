use std::sync::Arc;

use chashmap::CHashMap;

use crate::span::Spanned;

use super::ModulePath;

pub type ModuleContexts = CHashMap<Arc<ModulePath>, ModuleContext>;

#[derive(Debug, Default)]
pub struct ModuleContext {
	pub inclusions: Vec<Spanned<Inclusion>>,
}

#[derive(Debug)]
pub struct Inclusion {
	pub module_path: Arc<ModulePath>,
	pub terminal: InclusionTerminal,
}

#[derive(Debug)]
pub enum InclusionTerminal {
	Identifier(Arc<str>),
	Wildcard,
}
