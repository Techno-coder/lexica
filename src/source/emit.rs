use std::fmt::Display;

use colored::*;

use super::{Spanned, TextMap};

pub fn emit<E>(text_map: &TextMap, error: &Spanned<E>) where E: Display {
	println!("{} {}", "[Error]".bold().red(), error.node);

	text_map.prefix(&error.span, 1).iter()
		.for_each(|(line_index, line)| println!("{} | {}", format!("{:4}", line_index)
			.bright_blue().bold(), line));
	text_map.lines(&error.span).iter()
		.for_each(|(line_index, line)| println!("{} | {}", format!("{:4}", line_index)
			.bright_blue().bold(), line));

	let (line_prefix, specific) = text_map.line_offsets(&error.span);
	let line_prefix: String = line_prefix
		.chars()
		.map(|character| match character.is_whitespace() {
			true => character,
			false => ' ',
		}).collect();
	println!("     | {}{}", line_prefix, "^".repeat(specific.chars().count()).red());

	text_map.suffix(&error.span, 1).iter()
		.for_each(|(line_index, line)| println!("{} | {}", format!("{:4}", line_index)
			.bright_blue().bold(), line));
}