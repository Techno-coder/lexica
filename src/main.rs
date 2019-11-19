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

	let context = &Context::default();
	crate::declaration::module_root(context, "examples/mathematics/main.lx".to_owned().into());

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
