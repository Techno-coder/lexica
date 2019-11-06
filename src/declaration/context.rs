use std::sync::Arc;

use chashmap::CHashMap;

use crate::span::{Span, Spanned};

use super::ModulePath;

pub type ModuleContexts = CHashMap<Arc<ModulePath>, ModuleContext>;

#[derive(Debug)]
pub struct ModuleContext {
	pub inclusions: Vec<Spanned<Inclusion>>,
}

impl ModuleContext {
	pub fn new(module_path: Arc<ModulePath>, span: Span) -> Self {
		let mut inclusions = Vec::new();
		let terminal = InclusionTerminal::Wildcard;
		let inclusion = Inclusion { module_path, terminal };
		inclusions.push(Spanned::new(inclusion, span));
		Self { inclusions }
	}
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
