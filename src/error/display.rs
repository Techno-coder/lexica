use std::fmt::{Result, Write};

use crate::context::Context;
use crate::error::Diagnostic;
use crate::extension::{LineOffset, LineOffsets, StringExtension};
use crate::span::Span;

const GUTTER: &str = "    ";
const LOCATION_PREFIX: &str = "    -->";

pub fn display(write: &mut dyn Write, context: &Context, diagnostic: &Diagnostic) -> Result {
	writeln!(write, "{} {}", "[Error]", diagnostic.error.node)?;

	let span = diagnostic.error.span;
	let (location, has_error) = span.location_type(context);
	writeln!(write, "{} {}", LOCATION_PREFIX, location)?;

	if has_error {
		display_notes(write, diagnostic, true)?;
		return writeln!(write);
	}

	let source = span.source.get(context);
	let string = source.read_string().unwrap();
	let line_offsets = &string.line_offsets();
	let (&start_offset, &start_index) = line_offsets
		.range(..=span.byte_start).next_back().unwrap();

	display_prefix(write, string, line_offsets, start_offset)?;
	let (end_offset, end_index) = line_offsets.range(span.byte_end..).next()
		.map(|(offset, index)| (*offset, *index - 1)).unwrap_or((LineOffset(string.len()),
		line_offsets.len().saturating_sub(1)));
	match end_index > start_index {
		true => display_multiple(write, string, line_offsets, start_offset, end_offset, start_index),
		false => display_single(write, string, start_offset, end_offset, start_index, span),
	}?;

	display_suffix(write, string, line_offsets, end_offset)?;
	display_notes(write, diagnostic, false)?;
	writeln!(write)
}

fn display_multiple(write: &mut dyn Write, string: &str, line_offsets: &LineOffsets,
                    start_offset: LineOffset, end_offset: LineOffset, start_index: usize) -> Result {
	let mut lines = line_offsets.range(start_offset..end_offset).skip(1).peekable();
	let (&LineOffset(start_offset_end), _) = lines.peek().unwrap();
	writeln!(write, "{:4} / \t{}", start_index, &string[*start_offset..(start_offset_end - 1)])?;

	Ok(while let Some((&LineOffset(line_offset), line_index)) = lines.next() {
		match lines.peek() {
			Some((&LineOffset(offset_end), _)) => {
				let slice = &string[line_offset..offset_end - 1];
				writeln!(write, "{:4} | \t{}", line_index, slice)?;
			}
			None => {
				let slice = &string[line_offset..(*end_offset - 1)];
				writeln!(write, "{:4} \\ \t{}", line_index, slice)?;
			}
		}
	})
}

fn display_single(write: &mut dyn Write, string: &str, start_offset: LineOffset,
                  end_offset: LineOffset, start_index: usize, span: Span) -> Result {
	writeln!(write, "{:4} | \t{}", start_index,
		string.get(*start_offset..(*end_offset - 1)).unwrap_or(""))?;
	let initial: String = string[*start_offset..span.byte_start].chars()
		.map(|character| match character.is_whitespace() {
			true => character,
			false => ' ',
		}).collect();
	let specific_length = string.get(span.byte_start..span.byte_end)
		.map(|specific| specific.chars().count()).unwrap_or(1);
	writeln!(write, "     | \t{}{}", initial, "^".repeat(specific_length))
}

fn display_prefix(write: &mut dyn Write, string: &str, line_offsets: &LineOffsets,
                  start_offset: LineOffset) -> Result {
	match line_offsets.range(..start_offset).next_back() {
		Some((&LineOffset(prefix_offset), prefix_index)) =>
			writeln!(write, "{:4} | \t{}", prefix_index,
				&string[prefix_offset..(*start_offset - 1)]),
		None => writeln!(write, "{} |", GUTTER),
	}
}

fn display_suffix(write: &mut dyn Write, string: &str, line_offsets: &LineOffsets,
                  end_offset: LineOffset) -> Result {
	match line_offsets.range(*end_offset + 1..).next() {
		Some((&LineOffset(suffix_offset), suffix_index)) =>
			writeln!(write, "{:4} | \t{}", suffix_index - 1,
				&string[*end_offset..(suffix_offset - 1)]),
		None => writeln!(write, "{} |", GUTTER),
	}
}

fn display_notes(write: &mut dyn Write, diagnostic: &Diagnostic, internal_source: bool) -> Result {
	if !diagnostic.notes.is_empty() {
		if !internal_source {
			writeln!(write, "{} |", GUTTER)?;
		}

		diagnostic.notes.iter().try_for_each(|note|
			writeln!(write, "{} = {}", GUTTER, note))?;
	}
	Ok(())
}
