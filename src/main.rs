#![feature(mem_take)]
#![feature(option_unwrap_none)]
#![feature(never_type)]

mod error;
mod context;
mod declaration;
mod source;
mod span;
mod lexer;
mod parser;
mod node;
mod basic;
mod extension;
mod evaluation;
mod inference;
mod intrinsic;

fn main() {
	println!("Hello, world!");

	use std::sync::Arc;
	use crate::context::Context;
	use crate::declaration::*;
	use crate::span::*;
	use std::path::PathBuf;

	let path: Arc<PathBuf> = Arc::new("examples/mathematics/main.lx".to_owned().into());
	let module = ModulePending {
		expected_path: path.clone(),
		expected_module_path: None,
		declaration_span: Span::INTERNAL,
	};

	let context = &Context::default();
	context.modules_pending.insert(ModulePath::root(), module);

	let parameters = Vec::new();
	let result = context.emit(crate::evaluation::function(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "main".into(),
		})), Span::INTERNAL), parameters));
	result.map(|result| println!("{:#?}", result));

	context.errors.read().iter().for_each(|error| crate::error::display(context, error));
	println!("{:#?}", context);
}
