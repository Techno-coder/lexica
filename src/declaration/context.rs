use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::span::{Span, Spanned};

use super::{Declaration, ModulePath};

pub type ModuleContexts = RwLock<HashMap<Arc<ModulePath>, ModuleContext>>;

#[derive(Debug)]
pub struct ModuleContext {
	pub inclusions: Vec<Spanned<Inclusion>>,
	pub definitions: Vec<Definition>,
}

impl ModuleContext {
	pub fn new(module_path: Arc<ModulePath>, span: Span) -> Self {
		let mut inclusions = Vec::new();
		let terminal = InclusionTerminal::Wildcard;
		let inclusion = Inclusion { module_path, terminal };
		inclusions.push(Spanned::new(inclusion, span));
		ModuleContext { inclusions, definitions: Vec::new() }
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

#[derive(Debug)]
pub struct Definition {
	pub declaration: Declaration,
	pub methods: Vec<(Arc<str>, Spanned<Declaration>)>,
}

impl Definition {
	pub fn new(declaration: Declaration) -> Self {
		Definition { declaration, methods: Vec::new() }
	}
}
