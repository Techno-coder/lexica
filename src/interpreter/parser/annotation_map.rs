use std::collections::HashMap;

use super::AnnotationType;

/// Stores a register of annotation types keyed by their identifiers.
#[derive(Debug, Default)]
pub struct AnnotationMap {
	annotations: HashMap<String, Box<AnnotationType>>,
}

impl AnnotationMap {
	/// Adds an annotation type to the map.
	/// Existing annotations by the same identifier are overwritten.
	pub fn register(&mut self, identifier: String, annotation: Box<AnnotationType>) {
		self.annotations.insert(identifier, annotation);
	}

	/// Retrieves an annotation by the specified identifier key.
	pub fn get(&self, identifier: &str) -> Option<&AnnotationType> {
		self.annotations.get(identifier)
			.map(|annotation| annotation.as_ref())
	}
}
