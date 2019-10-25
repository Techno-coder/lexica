#![feature(mem_take)]
#![feature(option_unwrap_none)]

mod error;
mod context;
mod declaration;
mod source;
mod span;
mod lexer;
mod parser;
mod node;
mod extension;
mod inference;

fn main() {
	use std::sync::Arc;
	use crate::context::Context;
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
	context.modules_pending.write().insert(ModulePath::root(), module);
	let _ = context.emit(crate::inference::function(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "fibonacci".into(),
		})), Span::INTERNAL)));
	let _ = context.emit(crate::node::structure(context, &Spanned::new(Arc::new(
		StructurePath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root().append("vector".into()).append("vector".into()),
			identifier: "Vector".into(),
		})), Span::INTERNAL)));

	for error in context.errors.read().iter() {
		crate::error::display(context, error);
	}

	println!("{:#?}", context);
}
