use colored::Colorize;

use crate::compiler::TranslationMap;
use crate::interpreter::{CompilationUnit, TranslationUnit};
use crate::interpreter::parser::{AnnotationStore, ElementParser, OperationStore};
use crate::source::{Spanned, TextMap};

pub fn compile(source_map: &TextMap, translation_map: TranslationMap,
               operations: &OperationStore, annotations: &AnnotationStore)
               -> Option<CompilationUnit> {
	let text_map = TextMap::new(translation_map.text().to_owned());
	let unit = parse_bytecode(&source_map, &text_map, &translation_map, operations, annotations)?;
	let (unit, _metadata, errors) = crate::interpreter::compile(unit, operations);
	match errors.is_empty() {
		true => Some(unit),
		false => {
			for error in errors {
				emit_error(&source_map, &text_map, &translation_map, error);
			}
			None
		}
	}
}

pub fn parse_bytecode<'a>(source_map: &TextMap, text_map: &TextMap, translation_map: &'a TranslationMap,
                          operations: &'a OperationStore, annotations: &'a AnnotationStore)
                          -> Option<TranslationUnit<'a>> {
	let (mut parser_errors, mut elements) = (Vec::new(), Vec::new());
	let parser = ElementParser::new(translation_map.text(), annotations, operations);
	parser.for_each(|element| match element {
		Ok(element) => elements.push(element),
		Err(error) => parser_errors.push(error),
	});

	let (unit, mut errors) = crate::interpreter::parser::parse(elements, annotations);
	parser_errors.append(&mut errors);

	match parser_errors.is_empty() {
		true => Some(unit),
		false => {
			for error in parser_errors {
				emit_error(&source_map, &text_map, &translation_map, error);
			}
			None
		}
	}
}

pub fn emit_error<E>(source_map: &TextMap, text_map: &TextMap, translation_map: &TranslationMap,
                     mut error: Spanned<E>) where E: std::fmt::Display {
	crate::source::emit(&text_map, &error);
	println!("{}", format!("--> Error emitted from source: ").red().bold());
	translation_map.translate(&mut error);
	crate::source::emit_content(&source_map, &error);
	println!();
}
