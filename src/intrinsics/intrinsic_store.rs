use hashbrown::HashMap;

use super::*;

#[derive(Debug, Default)]
pub struct IntrinsicStore {
	intrinsics: HashMap<&'static str, Intrinsic>
}

impl IntrinsicStore {
	pub fn new() -> IntrinsicStore {
		let mut store = IntrinsicStore::default();
		store.register(trace::trace());
		store
	}

	pub fn register(&mut self, intrinsic: Intrinsic) {
		self.intrinsics.insert(intrinsic.identifier, intrinsic);
	}

	pub fn get(&self, identifier: &str) -> Option<&Intrinsic> {
		self.intrinsics.get(identifier)
	}

	pub fn intrinsics(&self) -> impl Iterator<Item=&Intrinsic> {
		self.intrinsics.values()
	}
}
