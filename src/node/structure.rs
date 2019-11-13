use std::collections::HashMap;
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::StructurePath;
use crate::node::AscriptionPattern;
use crate::span::Spanned;

pub type NodeStructures = CHashMap<Arc<StructurePath>, Arc<Structure>>;

#[derive(Debug, Clone)]
pub struct Structure {
	pub templates: Vec<Spanned<Arc<str>>>,
	pub fields: HashMap<Arc<str>, AscriptionPattern>,
}
