use std::sync::Arc;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::span::Spanned;

use super::{DeclarationError, ModuleContext, ModulePath};

/// Recursively loads the modules in the path.
pub fn load_modules(context: &Context, module_path: Arc<ModulePath>) -> Result<(), DeclarationError> {
	if let Some(parent) = &module_path.parent {
		load_modules(context, parent.clone())?;
	}

	let exists_declaration = context.module_contexts.read().contains_key(&module_path);
	match exists_declaration {
		true => Ok(()),
		false => {
			let exists_pending = context.modules_pending.read().contains_key(&module_path);
			match exists_pending {
				false => Err(DeclarationError::UndefinedModule(module_path)),
				true => {
					module_pending(context, module_path);
					Ok(())
				}
			}
		}
	}
}

/// Loads and parses a pending module. Panics if the module is not pending.
fn module_pending(context: &Context, module_path: Arc<ModulePath>) -> Option<()> {
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
		let error = DeclarationError::UndefinedModule(module_path);
		let diagnostic = Diagnostic::new(Spanned::new(error, module.declaration_span));
		let diagnostic = source_errors.into_iter().fold(diagnostic, |diagnostic, error|
			diagnostic.note(error.to_string()));
		return context.emit(Err(diagnostic));
	}

	context.module_contexts.write().insert(module_path.clone(),
		ModuleContext::default()).unwrap_none();
	sources.into_iter().try_for_each(|(source_key, physical_path)|
		super::SourceParse::parse(context, module_path.clone(), module.declaration_span,
			physical_path, source_key))
}

