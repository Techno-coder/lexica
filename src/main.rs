mod error;
mod context;
mod declaration;
mod source;
mod span;
mod lexer;
mod extension;

fn main() {
	use std::sync::Arc;
	use crate::context::Context;
	use crate::error::Diagnostic;
	use crate::declaration::*;
	use crate::span::*;
	use std::path::PathBuf;

	println!("Hello, world!");
	let path: Arc<PathBuf> = Arc::new("examples/mathematics/main.lx".to_owned().into());
	let module = ModulePending {
		expected_path: path.clone(),
		expected_module_path: None,
		declaration_span: Span::INTERNAL,
	};

	let context = &Context::default();
	let source_key = crate::source::source_key(context, &path).unwrap();
	let byte_end = source_key.get(context).data.len();

	let _: Option<()> = context.emit(Err(Diagnostic::new(Spanned::new(
		DeclarationError::ExpectedModuleTerminator, Span::INTERNAL))));
	let _: Option<()> = context.emit(Err(Diagnostic::new(Spanned::new(
		DeclarationError::ExpectedModuleTerminator, Span::new(source_key, 0, byte_end)))));

	context.modules_pending.write().insert(ModulePath::root(), module);
	let _ = module_pending(context, ModulePath::root());
	let _ = module_pending(context, ModulePath::root().append("vector".into()));

	for error in context.errors.read().iter() {
		crate::error::display(context, error);
	}

	println!("{:#?}", context);
}
