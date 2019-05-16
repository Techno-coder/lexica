use std::collections::HashMap;

use crate::source::Span;

use super::{CompileContext, CompileResult, GenericOperation, Operand, Operational, OperationKey};

pub type CompileFunction = for<'a, 'b> fn(&Span, &Vec<Operand<'a>>, &CompileContext<'a, 'b>)
                                          -> CompileResult<'a, GenericOperation>;

#[derive(Default)]
pub struct OperationStore {
	compile: HashMap<&'static str, CompileFunction>,
	arity: HashMap<&'static str, usize>,
}

impl OperationStore {
	/// Constructs an `OperationStore` with the default language operations.
	pub fn new() -> OperationStore {
		use super::operations::*;
		let mut store = Self::default();
		store.register::<Add>(OperationKey::Add);
		store.register::<AddImmediate>(OperationKey::AddImmediate);
		store.register::<Branch>(OperationKey::Branch);
		store.register::<BranchImmediate>(OperationKey::BranchImmediate);
		store.register::<Call>(OperationKey::Call);
		store.register::<CloneLocal>(OperationKey::Clone);
		store.register::<Discard>(OperationKey::Discard);
		store.register::<Drop>(OperationKey::Drop);
		store.register::<DropImmediate>(OperationKey::DropImmediate);
		store.register::<Exit>(OperationKey::Exit);
		store.register::<Jump>(OperationKey::Jump);
		store.register::<Recall>(OperationKey::Recall);
		store.register::<Reset>(OperationKey::Reset);
		store.register::<Return>(OperationKey::Return);
		store.register::<ReversalHint>(OperationKey::ReversalHint);
		store.register::<Swap>(OperationKey::Swap);
		store.register::<Minus>(OperationKey::Minus);
		store.register::<MinusImmediate>(OperationKey::MinusImmediate);
		store.register::<Restore>(OperationKey::Restore);
		store
	}

	pub fn register<T>(&mut self, identifier: OperationKey) where T: Operational {
		let identifier = identifier.into();
		self.compile.insert(identifier, T::compile);
		self.arity.insert(identifier, T::arity());
	}

	pub fn get(&self, identifier: &str) -> Option<(&'static str, &CompileFunction)> {
		self.compile.get_key_value(identifier)
			.map(|(key, value)| (*key, value))
	}

	pub fn arity(&self, identifier: &str) -> Option<usize> {
		self.arity.get(identifier).cloned()
	}
}


