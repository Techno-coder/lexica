use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{DeclarationError, ModulePath};

pub fn module_pending(context: &Context, module_path: Arc<ModulePath>) -> Option<()> {
	let module = context.modules_pending.write().remove(&module_path)
		.expect(&format!("Pending module: {:?}, does not exist", module_path));
	let mut sources = Vec::new();
	let mut source_errors = Vec::new();

	match crate::source::source_key(context, &module.expected_path) {
		Ok(source_key) => sources.push((source_key, &module.expected_path)),
		Err(error) => source_errors.push(error),
	}

	if let Some(expected_module_path) = &module.expected_module_path {
		match crate::source::source_key(context, &expected_module_path) {
			Ok(source_key) => sources.push((source_key, expected_module_path)),
			Err(error) => source_errors.push(error),
		}
	}

	if sources.is_empty() {
		let error = Spanned::new(DeclarationError::UndefinedModule, module.declaration_span);
		return context.emit(Err(Diagnostic::new(error)));
	}

	sources.into_iter().try_for_each(|(source_key, physical_path)|
		super::SourceParse::parse(context, module_path.clone(), module.declaration_span,
			physical_path, source_key))?;
	context.declarations_module.write().insert(module_path, module.into());
	Some(())
}

