use clap::{App, Arg, ArgMatches};

pub fn interface<'a>() -> ArgMatches<'a> {
	App::new("Lexica")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Lexica language compiler and interpreter")
		.arg(Arg::with_name("input")
			.required(true))
		.get_matches()
}
