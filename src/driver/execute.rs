use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::ArgMatches;

use crate::source::TextMap;

pub fn execute() -> Option<()> {
	let interface = super::interface();

	let source_map = parse_input(&interface);
	let intrinsic_store = crate::intrinsics::IntrinsicStore::new();
	let translation_map = super::translate(&source_map, &intrinsic_store)?;

	if interface.is_present("bytecode") {
		println!("{}", translation_map.text_map().text());
		return Some(());
	}

	let operation_store = crate::interpreter::parser::OperationStore::new();
	let annotation_store = crate::interpreter::parser::AnnotationStore::new();
	let compilation_unit = super::compile(&source_map, translation_map, &operation_store,
		&annotation_store, &intrinsic_store)?;

	let mut runtime = crate::interpreter::Runtime::new(compilation_unit)
		.expect("Failed to create runtime");

	let result = runtime.run(crate::interpreter::Direction::Advance);
	if let Err(error) = result {
		println!("Interpreter runtime error: {}", error);
	}
	println!("{:#?}", runtime.context());

	if interface.is_present("backtrack") {
		let result = runtime.run(crate::interpreter::Direction::Reverse);
		if let Err(error) = result {
			println!("Interpreter runtime error: {}", error);
		}
		println!("{:#?}", runtime.context());
	}

	Some(())
}

pub fn parse_input(interface: &ArgMatches) -> TextMap {
	let input = interface.value_of_os("input").unwrap();
	let mut file = File::open(Path::new(input))
		.expect(&format!("Specified file: {}, could not be opened", input.to_string_lossy()));

	let mut content = String::new();
	file.read_to_string(&mut content)
		.expect(&format!("Failed to read file: {}", input.to_string_lossy()));
	TextMap::new(content)
}
