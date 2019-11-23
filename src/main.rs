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
mod interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let arguments: Vec<_> = std::env::args_os().collect();
	let root_path = match arguments.get(1) {
		Some(path) => std::path::PathBuf::from(path.clone()),
		None => return Ok(eprintln!("The module root file is not specified")),
	};

	let context = &context::Context::default();
	crate::declaration::module_root(context, root_path);
	crate::node::load_definitions(context);
	interface::interface(context)
}
