use std::collections::HashMap;

use super::{AnnotationType, ParserError, ParserResult};

#[derive(Debug, Default)]
pub struct AnnotationMap {
	annotations: HashMap<String, Box<AnnotationType>>,
}

impl AnnotationMap {
	pub fn register(&mut self, identifier: String, annotation: Box<AnnotationType>) {
		self.annotations.insert(identifier, annotation);
	}

	pub fn get(&self, identifier: &str) -> Option<&AnnotationType> {
		self.annotations.get(identifier)
		    .map(|annotation| annotation.as_ref())
	}
}
