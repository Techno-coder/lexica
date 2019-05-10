use std::iter::Peekable;

use crate::source::{Span, Spanned};

use super::{AnnotationStore, Element, ElementParser, FunctionOffset, OperationStore,
            ParserContext, ParserError, TranslationFunction, TranslationUnit};

pub fn parse<'a>(text: &'a str, annotations: &'a AnnotationStore, operations: &'a OperationStore)
                 -> (TranslationUnit<'a>, Vec<Spanned<ParserError<'a>>>) {
	let mut unit = TranslationUnit::default();
	let mut elements = ElementParser::new(text, annotations, operations).peekable();
	let mut context = ParserContext::default();

	collect_annotations(&mut elements, &mut context);
	while let Some(element) = elements.next() {
		let error_count = context.errors.len();
		match element {
			Ok(element) => parse_element(element, &mut context, &mut unit),
			Err(error) => context.errors.push(error),
		}

		match error_count == context.errors.len() {
			true => process_annotations(annotations, &mut context, &mut unit),
			false => context.pending_annotations.clear(),
		}
		collect_annotations(&mut elements, &mut context);
	}
	(unit, context.errors)
}

fn parse_element<'a>(element: Spanned<Element<'a>>, context: &mut ParserContext<'a>,
                     unit: &mut TranslationUnit<'a>) {
	match element.node.clone() {
		Element::Function(identifier) => parse_function(element.span.clone(), identifier, context, unit),
		Element::Label(label) => parse_label(element.span.clone(), label, context, unit),
		Element::Instruction(instruction) => {
			let instruction = Spanned::new(instruction, element.span.clone());
			context.pending_function(unit).instructions.push(instruction);
		}
		Element::Annotation(_) => panic!("Annotation exists but not parsed"),
	}
	context.last_element = Some(element);
}

fn parse_function<'a>(span: Span, identifier: &'a str, context: &mut ParserContext<'a>,
                      unit: &mut TranslationUnit) {
	match unit.functions.contains_key(identifier) {
		true => context.errors.push(Spanned::new(ParserError::DuplicateFunction(identifier), span)),
		false => {
			let function = TranslationFunction::default();
			unit.functions.insert(identifier.to_owned(), function);
			context.pending_function = Some(identifier);
		}
	}
}

fn parse_label<'a>(span: Span, label: &'a str, context: &mut ParserContext<'a>,
                   unit: &mut TranslationUnit<'a>) {
	let pending_function = context.pending_function(unit);
	let target = FunctionOffset(pending_function.instructions.len());
	match pending_function.labels.contains_key(label) {
		true => context.errors.push(Spanned::new(ParserError::DuplicateLabel(label), span)),
		false => {
			pending_function.labels.insert(label.to_owned(), target);
		}
	}
}

fn process_annotations(annotations: &AnnotationStore, context: &mut ParserContext,
                       unit: &mut TranslationUnit) {
	for annotation in &context.pending_annotations {
		let annotation_type = match annotations.get(annotation.identifier) {
			Some(annotation_type) => annotation_type,
			None => {
				let error = ParserError::InvalidAnnotation(annotation.identifier);
				context.errors.push(Spanned::new(error, annotation.span.clone()));
				continue;
			}
		};

		if let Err(error) = annotation_type.annotate(&annotation, context, unit) {
			context.errors.push(error);
		}
	}
	context.pending_annotations.clear();
}

fn collect_annotations<'a>(elements: &mut Peekable<ElementParser<'a>>, context: &mut ParserContext<'a>) {
	while let Some(Ok(annotation)) = elements.peek() {
		match annotation.node {
			Element::Annotation(_) => match elements.next() {
				Some(Ok(element)) => {
					context.last_element = Some(element.clone());
					match element.node {
						Element::Annotation(annotation) => {
							let annotation = Spanned::new(annotation, element.span);
							context.pending_annotations.push(annotation);
						}
						_ => unreachable!(),
					}
				}
				_ => unreachable!(),
			}
			_ => break,
		}
	}
}
