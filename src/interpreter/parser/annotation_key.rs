#[derive(Debug)]
pub enum AnnotationKey {
	Local,
	Other(&'static str),
}

impl Into<&'static str> for AnnotationKey {
	fn into(self) -> &'static str {
		match self {
			AnnotationKey::Local => "local",
			AnnotationKey::Other(other) => other,
		}
	}
}
