use std::ops::{Index, IndexMut};

use super::{InterpreterError, InterpreterResult, Primitive};

#[derive(Debug, Clone, Default)]
pub struct LocalTable {
	locals: Vec<Primitive>,
}

impl LocalTable {
	pub fn register(&mut self, local: Primitive) {
		self.locals.push(local);
	}

	pub fn local(&self, target: &LocalTarget) -> InterpreterResult<&Primitive> {
		let LocalTarget(index) = target;
		self.locals.get(*index).ok_or(InterpreterError::InvalidLocal)
	}

	pub fn local_mut(&mut self, target: &LocalTarget) -> InterpreterResult<&mut Primitive> {
		let LocalTarget(index) = target;
		self.locals.get_mut(*index).ok_or(InterpreterError::InvalidLocal)
	}
}

impl Index<&LocalTarget> for LocalTable {
	type Output = Primitive;

	fn index(&self, index: &LocalTarget) -> &Self::Output {
		self.local(index).expect("Local target is out of bounds")
	}
}

impl IndexMut<&LocalTarget> for LocalTable {
	fn index_mut(&mut self, index: &LocalTarget) -> &mut Self::Output {
		self.local_mut(index).expect("Local target is out of bounds")
	}
}

#[derive(Debug)]
pub struct LocalTarget(pub usize);
