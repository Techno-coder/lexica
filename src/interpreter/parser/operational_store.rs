use std::collections::HashMap;

use crate::source::Span;

use super::{CompileContext, CompileResult, GenericOperation, Operand, Operational};

pub type OperationParser = for<'a> fn(&Span, &Vec<Operand<'a>>, &CompileContext)
                                      -> CompileResult<'a, GenericOperation>;

#[derive(Default)]
pub struct OperationalStore {
	operations: HashMap<&'static str, OperationParser>,
}

impl OperationalStore {
	/// Constructs an `OperationStore` with the default language operations.
	pub fn new() -> OperationalStore {
		// TODO
//		use super::operations::*;
		let mut store = Self::default();
//		store.register::<Add>("add");
//		store.register::<AddImmediate>("add.i");
//		store.register::<Branch>("branch");
//		store.register::<BranchImmediate>("branch.i");
//		store.register::<Call>("call");
//		store.register::<CloneLocal>("clone");
//		store.register::<Discard>("discard");
//		store.register::<Drop>("drop");
//		store.register::<DropImmediate>("drop.i");
//		store.register::<Exit>("exit");
//		store.register::<Jump>("jump");
//		store.register::<Recall>("recall");
//		store.register::<Reset>("reset");
//		store.register::<Return>("return");
//		store.register::<ReversalHint>("*");
//		store.register::<Swap>("swap");
//		store.register::<Minus>("minus");
//		store.register::<MinusImmediate>("minus.i");
//		store.register::<Restore>("restore");
		store
	}

	pub fn register<T>(&mut self, identifier: &'static str) where T: Operational {
		self.operations.insert(identifier, T::compile);
	}

	pub fn get(&self, identifier: &str) -> Option<(&'static str, &OperationParser)> {
		self.operations.get_key_value(identifier)
			.map(|(key, value)| (*key, value))
	}
}

