use clap::{App, Arg, ArgMatches};

pub fn interface<'a>() -> ArgMatches<'a> {
	App::new("Lexica")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Lexica language compiler and interpreter")
		.arg(Arg::with_name("input")
			.required(true))
		.arg(Arg::with_name("bytecode")
			.help("Emits the bytecode translation of the source")
			.short("b"))
		.arg(Arg::with_name("backtrack")
			.help("Reverses the context after execution has halted")
			.short("u"))
		.get_matches()
}
