use std::collections::BTreeMap;

use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::StringExtension;
use crate::span::Span;

const GUTTER: &str = "    ";
const LOCATION_GUTTER: &str = "    -->";

pub fn display(context: &Context, diagnostic: &Diagnostic) {
	println!("{} {}", "[Error]", diagnostic.error.node);

	let span = diagnostic.error.span;
	let (location, has_error) = span.location(context);
	println!("{} {}", LOCATION_GUTTER, location);

	if has_error {
		return println!();
	}

	let source = span.source.get(context);
	let string = source.read_string().unwrap();
	let line_offsets = &string.line_offsets();
	let (&start_offset, &start_index) = line_offsets
		.range(..=span.byte_start).next_back().unwrap();

	display_prefix(string, line_offsets, start_offset);
	let (end_offset, end_index) = line_offsets.range(span.byte_end..).next()
		.map(|(offset, index)| (*offset, *index - 1)).unwrap_or((string.len(), line_offsets.len()));
	match end_index > start_index {
		true => display_multiple(string, line_offsets, start_offset, end_offset, start_index),
		false => display_single(string, start_offset, end_offset, start_index, span),
	}

	display_suffix(string, line_offsets, end_offset);
	display_notes(diagnostic);
	println!();
}

fn display_multiple(string: &str, line_offsets: &BTreeMap<usize, usize>,
                    start_offset: usize, end_offset: usize, start_index: usize) {
	let mut lines = line_offsets.range(start_offset..end_offset).skip(1).peekable();
	let (&start_offset_end, _) = lines.peek().unwrap();
	println!("{} / \t{}", format!("{:4}", start_index), &string[start_offset..(start_offset_end - 1)]);

	while let Some((&line_offset, line_index)) = lines.next() {
		match lines.peek() {
			Some((&offset_end, _)) => {
				let slice = &string[line_offset..(offset_end - 1)];
				println!("{} | \t{}", format!("{:4}", line_index), slice);
			}
			None => {
				let slice = &string[line_offset..(end_offset - 1)];
				println!("{} \\ \t{}", format!("{:4}", line_index), slice);
			}
		}
	}
}

fn display_single(string: &str, start_offset: usize, end_offset: usize, start_index: usize, span: Span) {
	println!("{} | \t{}", format!("{:4}", start_index), &string[start_offset..(end_offset - 1)]);
	let initial: String = string[start_offset..span.byte_start].chars()
		.map(|character| match character.is_whitespace() {
			true => character,
			false => ' ',
		}).collect();
	let specific_length = string[span.byte_start..span.byte_end].chars().count();
	println!("     | \t{}{}", initial, "^".repeat(specific_length));
}

fn display_prefix(string: &str, line_offsets: &BTreeMap<usize, usize>, start_offset: usize) {
	match line_offsets.range(..start_offset).next_back() {
		Some((&prefix_offset, prefix_index)) =>
			println!("{} | \t{}", format!("{:4}", prefix_index),
				&string[prefix_offset..(start_offset - 1)]),
		None => println!("{} |", GUTTER),
	}
}

fn display_suffix(string: &str, line_offsets: &BTreeMap<usize, usize>, end_offset: usize) {
	match line_offsets.range(end_offset + 1..).next() {
		Some((&suffix_offset, suffix_index)) =>
			println!("{} | \t{}", format!("{:4}", suffix_index - 1),
				&string[end_offset..(suffix_offset - 1)]),
		None => println!("{} |", GUTTER),
	}
}

fn display_notes(diagnostic: &Diagnostic) {
	if !diagnostic.notes.is_empty() {
		println!("{} |", GUTTER);
		diagnostic.notes.iter()
			.for_each(|note| println!("{} = {}", GUTTER, note))
	}
}
