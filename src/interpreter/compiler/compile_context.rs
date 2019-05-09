use crate::source::Spanned;

use super::{CompileError, TranslationFunction, TranslationUnit};

#[derive(Debug)]
pub struct CompileContext<'a> {
	/// The current `TranslationUnit` that is being processed.
	pub unit: TranslationUnit<'a>,
	/// The current function that is being processed.
	pub pending_function: Option<&'a TranslationFunction<'a>>,
	/// The errors encountered by the compiler.
	pub errors: Vec<Spanned<CompileError>>,
}

impl<'a> CompileContext<'a> {
	pub fn new(unit: TranslationUnit<'a>) -> Self {
		Self {
			unit,
			pending_function: None,
			errors: Vec::new(),
		}
	}

	pub fn pending_function(&self) -> &'a TranslationFunction<'a> {
		self.pending_function.as_ref().expect("No function has been reached")
	}
}
