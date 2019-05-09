use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CompileMetadata {
	pub function_indexes: HashMap<String, usize>,
}