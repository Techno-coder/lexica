use hashbrown::HashMap;

use super::{AnnotationKey, Annotator};

/// Stores a register of annotation types keyed by their identifiers.
#[derive(Debug, Default)]
pub struct AnnotationStore {
	annotations: HashMap<&'static str, Box<Annotator>>,
}

impl AnnotationStore {
	/// Constructs an `AnnotationStore` with the default language annotators.
	pub fn new() -> AnnotationStore {
		use super::annotations::*;
		let mut store = AnnotationStore::default();
		store.register(AnnotationKey::Local, Box::new(LocalAnnotation));
		store
	}

	/// Adds an annotation type to the store.
	/// Existing annotations by the same identifier are overwritten.
	pub fn register(&mut self, identifier: AnnotationKey, annotation: Box<Annotator>) {
		let identifier = identifier.into();
		self.annotations.insert(identifier, annotation);
	}

	/// Retrieves an annotation by the specified identifier key.
	pub fn get(&self, identifier: &str) -> Option<&Annotator> {
		self.annotations.get(identifier)
			.map(|annotation| annotation.as_ref())
	}
}
