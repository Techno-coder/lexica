use std::collections::HashMap;

use super::{FunctionTarget, TranslationUnit};

#[derive(Debug, Default)]
pub struct CompileMetadata {
	pub function_targets: HashMap<String, FunctionTarget>,
}

impl CompileMetadata {
	/// Creates a metadata object with function targets precomputed.
	pub fn construct(unit: &TranslationUnit) -> CompileMetadata {
		let mut metadata = CompileMetadata::default();
		for (index, identifier) in unit.functions.keys().enumerate() {
			metadata.function_targets.insert(identifier.to_owned(), FunctionTarget(index));
		}
		metadata
	}
}
