use std::collections::HashMap;

use crate::source::{Span, Spanned};

use super::{AnnotationMap, Element, ElementParser, Instruction, InstructionTarget, LocalTable,
            Operation, ParserContext, ParserError, TranslationFunctionLabel, TranslationUnit};

pub fn parse<'a>(text: &'a str, annotation_map: &'a AnnotationMap)
                 -> Result<TranslationUnit, Vec<Spanned<ParserError<'a>>>> {
	let mut errors: Vec<Spanned<ParserError<'a>>> = Vec::new();
	let mut unit = TranslationUnit::default();

	let (elements, element_errors) = ElementParser::new(text, annotation_map)
		.partition::<Vec<_>, _>(|element| element.is_ok());
	let elements: Vec<Spanned<Element>> = elements.into_iter().map(Result::unwrap).collect();
	errors.extend(element_errors.into_iter().map(Result::unwrap_err));

	let mut context = match elements.get(0) {
		Some(element) => ParserContext::new(element.clone()),
		None => return match errors.is_empty() {
			true => Ok(unit),
			false => Err(errors),
		},
	};

	errors.extend(labels(&mut unit, &elements).into_iter());
	errors.extend(reverse_labels(&mut unit, &elements).into_iter());

	let mut instruction_index = 0;
	for element in elements {
		if element.advances_counter() {
			instruction_index += 1;
		}

		context.last_element = element.clone();
		match &element.node {
			Element::Annotation(annotation) => {
				let annotation = Spanned::new(annotation.clone(), element.span);
				context.pending_annotations.push(annotation);
			}
			Element::ReversalHint => unit.instructions.push(Instruction {
				operation: Operation::ReversalHint,
				direction: None,
				polarization: None,
			}),
			Element::FunctionLabel(function) => context.last_function_label = Some(function),
			Element::Label(label) => context.last_function_label = Some(label),
			Element::Instruction(instruction) => {
				use super::match_operation;
				let operation = match_operation(&element.span, &instruction.operation, &instruction.operands,
				                                &context, &unit);
				match operation {
					Ok(operation) => {
						unit.instructions.push(Instruction {
							operation,
							direction: instruction.direction,
							polarization: instruction.polarization,
						});
					}
					Err(error) => errors.push(error),
				};
			}
			_ => (),
		}

		match element.node {
			Element::Annotation(_) => (),
			other => {
				for annotation in &context.pending_annotations {
					let annotation_type = annotation_map.get(annotation.identifier).unwrap();
					if let Err(error) = annotation_type.annotate(&annotation, &context, &mut unit) {
						errors.push(error);
					}
				}
				context.pending_annotations.clear();
			}
		}
	}

	match errors.is_empty() {
		true => Ok(unit),
		false => Err(errors),
	}
}

fn labels<'a>(unit: &mut TranslationUnit, elements: &Vec<Spanned<Element<'a>>>)
              -> Vec<Spanned<ParserError<'a>>> {
	let mut errors = Vec::new();
	let mut instruction_index = 0;
	let mut last_label = None;
	for element in elements {
		if element.advances_counter() {
			instruction_index += 1;
		}

		match element.node {
			Element::Label(label) => match unit.labels.contains_key(label) {
				true => errors.push(Spanned::new(ParserError::DuplicateLabel(label), element.span.clone())),
				false => {
					let target = InstructionTarget(instruction_index);
					unit.labels.insert(label.to_owned(), (target, HashMap::new()));
					last_label = Some(label);
				}
			}
			Element::LocalLabel(label) => match last_label {
				Some(last_label) => {
					let (_, local_labels) = unit.labels.get_mut(last_label).unwrap();
					match local_labels.contains_key(label) {
						true => errors.push(Spanned::new(ParserError::DuplicateLocalLabel(label), element.span.clone())),
						false => {
							let target = InstructionTarget(instruction_index);
							local_labels.insert(label.to_owned(), target);
						}
					}
				}
				None => errors.push(Spanned::new(ParserError::LabelMissingContext(label), element.span.clone())),
			}
			Element::FunctionLabel(label) => match unit.labels.contains_key(label) {
				true => errors.push(Spanned::new(ParserError::DuplicateLabel(label), element.span.clone())),
				false => {
					let target = InstructionTarget(instruction_index);
					unit.functions.insert(label.to_owned(), TranslationFunctionLabel {
						locals: LocalTable::default(),
						target: target.clone(),
						reverse_target: None,
					});
					unit.labels.insert(label.to_owned(), (target, HashMap::new()));
					last_label = Some(label);
				}
			}
			_ => (),
		}
	}
	errors
}

fn reverse_labels<'a>(unit: &mut TranslationUnit, elements: &Vec<Spanned<Element<'a>>>)
                      -> Vec<Spanned<ParserError<'a>>> {
	let mut errors = Vec::new();
	let mut instruction_index = 0;
	for element in elements {
		if element.advances_counter() {
			instruction_index += 1;
		}

		match element.node {
			Element::ReverseLabel(label) => {
				match unit.functions.get_mut(label) {
					Some(function) => match &function.reverse_target {
						Some(target) => errors.push(Spanned::new(ParserError::DuplicateReverseLabel(label),
						                                         element.span.clone())),
						None => function.reverse_target = Some(InstructionTarget(instruction_index)),
					}
					None => errors.push(Spanned::new(ParserError::IsolatedReverseLabel(label), element.span.clone()))
				}
			}
			_ => (),
		}
	}
	errors
}
