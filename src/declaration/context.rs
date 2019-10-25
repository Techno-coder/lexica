use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::ModulePath;

pub type ModuleContexts = RwLock<HashMap<Arc<ModulePath>, ModuleContext>>;

#[derive(Debug, Default)]
pub struct ModuleContext {
	pub inclusions: Vec<Inclusion>,
}

#[derive(Debug)]
pub struct Inclusion {
	pub module: Arc<ModulePath>,
	pub terminal: InclusionTerminal,
}

#[derive(Debug)]
pub enum InclusionTerminal {
	Identifier(Arc<str>),
	Wildcard,
}
