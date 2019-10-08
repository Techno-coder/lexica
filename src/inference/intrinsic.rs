use std::sync::Arc;

use crate::declaration::{DeclarationPath, ModulePath, StructurePath};

use super::InferenceType;

pub fn tuple() -> StructurePath {
	structure("tuple")
}

pub fn unit() -> Arc<InferenceType> {
	inference_type("unit")
}

pub fn truth() -> Arc<InferenceType> {
	inference_type("truth")
}

fn inference_type(identifier: &'static str) -> Arc<InferenceType> {
	Arc::new(InferenceType::Instance(structure(identifier), Vec::new()))
}

fn structure(identifier: &'static str) -> StructurePath {
	StructurePath(DeclarationPath {
		module_path: ModulePath::intrinsic(),
		identifier: identifier.into(),
	})
}
