use std::collections::HashMap;
use std::sync::Arc;

use chashmap::CHashMap;

use crate::declaration::StructurePath;
use crate::node::AscriptionPattern;

pub type NodeStructures = CHashMap<Arc<StructurePath>, Arc<Structure>>;

#[derive(Debug, Clone)]
pub struct Structure {
	pub fields: HashMap<Arc<str>, AscriptionPattern>,
}
