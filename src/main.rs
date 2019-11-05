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
	use crate::basic::*;
	use std::collections::HashMap;
	use std::path::PathBuf;

	let path: Arc<PathBuf> = Arc::new("examples/mathematics/main.lx".to_owned().into());
	let module = ModulePending {
		expected_path: path.clone(),
		expected_module_path: None,
		declaration_span: Span::INTERNAL,
	};

	let context = &Context::default();
	context.modules_pending.insert(ModulePath::root(), module);
	let _ = context.emit(crate::inference::function(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "fibonacci".into(),
		})), Span::INTERNAL)));
	let _ = context.emit(crate::node::structure(context, &Spanned::new(Arc::new(
		StructurePath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root().push("vector".into()).push("vector".into()),
			identifier: "Vector".into(),
		})), Span::INTERNAL)));

	let mut parameters = HashMap::new();
	parameters.insert("n".into(), Item::Unsigned64(35));

	let result = context.emit(crate::evaluation::evaluate(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "fibonacci".into(),
		})), Span::INTERNAL), parameters));
	result.map(|result| println!("{}", result));

	let _function = context.emit(crate::parser::function(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "absolute_value".into(),
		})), Span::INTERNAL)));
	println!("{:#?}", _function);

	for error in context.errors.read().iter() {
		crate::error::display(context, error);
	}

	println!("{:#?}", context);
}
