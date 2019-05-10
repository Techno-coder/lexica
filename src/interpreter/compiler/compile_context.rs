use crate::source::Spanned;

use super::{CompileError, CompileMetadata, TranslationFunction, TranslationUnit};

#[derive(Debug)]
pub struct CompileContext<'a, 'b> {
	/// The current `TranslationUnit` that is being processed.
	pub unit: TranslationUnit<'a>,
	/// The compilation metadata associated with the current `CompilationUnit`.
	pub metadata: CompileMetadata,
	/// The current function that is being processed.
	pub pending_function: Option<&'b TranslationFunction<'a>>,
	/// The errors encountered by the compiler.
	pub errors: Vec<Spanned<CompileError<'a>>>,
}

impl<'a, 'b> CompileContext<'a, 'b> {
	pub fn new(unit: TranslationUnit<'a>) -> Self {
		let metadata = CompileMetadata::construct(&unit);
		Self {
			unit,
			metadata,
			pending_function: None,
			errors: Vec::new(),
		}
	}
}
