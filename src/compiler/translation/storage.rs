use std::fmt;

use crate::node::{AccessorTarget, Identifier, VariableTarget};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct StorageTarget<'a>(pub Identifier<'a>, pub usize, pub String);

impl<'a> From<VariableTarget<'a>> for StorageTarget<'a> {
	fn from(target: VariableTarget<'a>) -> Self {
		let VariableTarget(identifier, generation, AccessorTarget(accessor)) = target;
		StorageTarget(identifier, generation, accessor.to_owned())
	}
}

impl<'a> fmt::Display for StorageTarget<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let StorageTarget(identifier, generation, accessor) = &self;
		VariableTarget(identifier.clone(), *generation, AccessorTarget(&accessor)).fmt(f)
	}
}
