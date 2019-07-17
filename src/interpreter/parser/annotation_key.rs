#[derive(Debug)]
pub enum AnnotationKey {
	Local,
}

impl Into<&'static str> for AnnotationKey {
	fn into(self) -> &'static str {
		match self {
			AnnotationKey::Local => "local",
		}
	}
}
