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

fn main() {
	println!("Hello, world!");

	use std::sync::Arc;
	use crate::context::Context;
	use crate::declaration::*;
	use crate::evaluation::*;
	use crate::node::*;
	use crate::span::*;
	use crate::basic::*;
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

	let _basic = context.emit(crate::basic::basic_function(context, &Spanned::new(Arc::new(
		FunctionPath(crate::declaration::DeclarationPath {
			module_path: ModulePath::root(),
			identifier: "math_expression".into(),
		})), Span::INTERNAL)));
	println!("{:#?}", _basic);

	let basic = _basic.unwrap();
	let mut frame = EvaluationFrame::new(basic);
	frame.context.insert(Variable("a".into(), 0), Item::Unsigned64(2));
	frame.context.insert(Variable("b".into(), 0), Item::Unsigned64(3));
	frame.context.insert(Variable("c".into(), 0), Item::Unsigned64(5));
	let mut evaluation = EvaluationContext::new(frame);
	loop {
		println!("{:#?}", evaluation.advance());
	}

	for error in context.errors.read().iter() {
		crate::error::display(context, error);
	}

	println!("{:#?}", context);
}
