use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::declaration::StructurePath;
use crate::node::AscriptionPattern;

pub type NodeStructures = RwLock<HashMap<Arc<StructurePath>, Arc<Structure>>>;

#[derive(Debug, Clone)]
pub struct Structure {
	pub fields: HashMap<Arc<str>, AscriptionPattern>,
}

impl Structure {
	pub fn new(fields: HashMap<Arc<str>, AscriptionPattern>) -> Self {
		Structure { fields }
	}
}
