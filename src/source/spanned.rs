use std::ops::{Deref, DerefMut};

use super::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
	pub node: T,
	pub span: Span,
}

impl<T> Spanned<T> {
	pub fn new(node: T, span: Span) -> Self {
		Self { node, span }
	}

	pub fn map<'a, F, R>(&'a self, function: F) -> Spanned<R> where F: FnOnce(&'a T) -> R {
		let node = function(&self.node);
		Spanned::new(node, self.span.clone())
	}
}

impl<T> Deref for Spanned<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.node
	}
}

impl<T> DerefMut for Spanned<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.node
	}
}

impl<T> AsRef<T> for Spanned<T> {
	fn as_ref(&self) -> &T {
		&self.node
	}
}

impl<T> AsMut<T> for Spanned<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.node
	}
}
