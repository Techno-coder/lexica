use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::{LineOffset, LineOffsets, StringExtension};
use crate::span::Span;

const GUTTER: &str = "    ";
const LOCATION_PREFIX: &str = "    -->";

pub fn display(context: &Context, diagnostic: &Diagnostic) {
	println!("{} {}", "[Error]", diagnostic.error.node);

	let span = diagnostic.error.span;
	let (location, has_error) = span.location_type(context);
	println!("{} {}", LOCATION_PREFIX, location);

	if has_error {
		display_notes(diagnostic, true);
		return println!();
	}

	let source = span.source.get(context);
	let string = source.read_string().unwrap();
	let line_offsets = &string.line_offsets();
	let (&start_offset, &start_index) = line_offsets
		.range(..=span.byte_start).next_back().unwrap();

	display_prefix(string, line_offsets, start_offset);
	let (end_offset, end_index) = line_offsets.range(span.byte_end..).next()
		.map(|(offset, index)| (*offset, *index - 1)).unwrap_or((LineOffset(string.len()),
		line_offsets.len().saturating_sub(1)));
	match end_index > start_index {
		true => display_multiple(string, line_offsets, start_offset, end_offset, start_index),
		false => display_single(string, start_offset, end_offset, start_index, span),
	}

	display_suffix(string, line_offsets, end_offset);
	display_notes(diagnostic, false);
	println!();
}

fn display_multiple(string: &str, line_offsets: &LineOffsets,
                    start_offset: LineOffset, end_offset: LineOffset, start_index: usize) {
	let mut lines = line_offsets.range(start_offset..end_offset).skip(1).peekable();
	let (&LineOffset(start_offset_end), _) = lines.peek().unwrap();
	println!("{} / \t{}", format!("{:4}", start_index), &string[*start_offset..(start_offset_end - 1)]);

	while let Some((&LineOffset(line_offset), line_index)) = lines.next() {
		match lines.peek() {
			Some((&LineOffset(offset_end), _)) => {
				let slice = &string[line_offset..offset_end - 1];
				println!("{} | \t{}", format!("{:4}", line_index), slice);
			}
			None => {
				let slice = &string[line_offset..(*end_offset - 1)];
				println!("{} \\ \t{}", format!("{:4}", line_index), slice);
			}
		}
	}
}

fn display_single(string: &str, start_offset: LineOffset, end_offset: LineOffset,
                  start_index: usize, span: Span) {
	println!("{} | \t{}", format!("{:4}", start_index),
		string.get(*start_offset..(*end_offset - 1)).unwrap_or(""));
	let initial: String = string[*start_offset..span.byte_start].chars()
		.map(|character| match character.is_whitespace() {
			true => character,
			false => ' ',
		}).collect();
	let specific_length = string.get(span.byte_start..span.byte_end)
		.map(|specific| specific.chars().count()).unwrap_or(1);
	println!("     | \t{}{}", initial, "^".repeat(specific_length));
}

fn display_prefix(string: &str, line_offsets: &LineOffsets, start_offset: LineOffset) {
	match line_offsets.range(..start_offset).next_back() {
		Some((&LineOffset(prefix_offset), prefix_index)) =>
			println!("{} | \t{}", format!("{:4}", prefix_index),
				&string[prefix_offset..(*start_offset - 1)]),
		None => println!("{} |", GUTTER),
	}
}

fn display_suffix(string: &str, line_offsets: &LineOffsets, end_offset: LineOffset) {
	match line_offsets.range(*end_offset + 1..).next() {
		Some((&LineOffset(suffix_offset), suffix_index)) =>
			println!("{} | \t{}", format!("{:4}", suffix_index - 1),
				&string[*end_offset..(suffix_offset - 1)]),
		None => println!("{} |", GUTTER),
	}
}

fn display_notes(diagnostic: &Diagnostic, internal_source: bool) {
	if !diagnostic.notes.is_empty() {
		if !internal_source {
			println!("{} |", GUTTER);
		}

		diagnostic.notes.iter()
			.for_each(|note| println!("{} = {}", GUTTER, note))
	}
}
