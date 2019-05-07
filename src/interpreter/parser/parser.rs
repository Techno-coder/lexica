use std::collections::HashMap;

use crate::source::Spanned;

use super::{AnnotationMap, Element, ElementParser, Instruction, InstructionTarget,
            LocalTable, OperationalStore, ParserContext, ParserError, TranslationFunctionLabel,
            TranslationUnit};

/// Parses byte code into a `TranslationUnit`.
/// Additionally, verifies that all instructions are valid.
pub fn parse<'a>(text: &'a str, annotation_map: &'a AnnotationMap, store: &OperationalStore)
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

	for element in elements {
		context.last_element = element.clone();
		match &element.node {
			Element::Annotation(annotation) => {
				let annotation = Spanned::new(annotation.clone(), element.span);
				context.pending_annotations.push(annotation);
			}
			Element::FunctionLabel(function) => context.last_function_label = Some(function),
			Element::Label(label) => context.last_function_label = Some(label),
			Element::Instruction(instruction) => {
				let (identifier, constructor) = store.get(&format!("{}", instruction.operation))
					.expect("Invalid instruction operation parsed");
				let operation = constructor(&element.span, &instruction.operands, &context, &unit);

				// TODO: Consider reversibility of instructions (Call)
//				match &instruction.direction {
//					Direction::Advance => (),
//					_ => match &operation {
//						Ok(RefactorOperation::Call(call)) => match call.reversible() {
//							false => {
//								let error = ParserError::IrreversibleCall;
//								errors.push(Spanned::new(error, element.span));
//								continue;
//							}
//							_ => (),
//						}
//						_ => (),
//					}
//				}

				match operation {
					Ok(operation) => {
						unit.instructions.push(Instruction {
							identifier,
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
			_ => {
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

/// Adds labels and label definitions to the `TranslationUnit`.
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

/// Adds and verifies reverse function labels to the `TranslationUnit`.
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
						Some(_) => errors.push(element.map(|_| ParserError::DuplicateReverseLabel(label))),
						None => match instruction_index > 0 {
							true => function.reverse_target = Some(InstructionTarget(instruction_index - 1)),
							false => errors.push(element.map(|_| ParserError::InvalidReverseLabelPosition(label))),
						}
					}
					None => errors.push(Spanned::new(ParserError::IsolatedReverseLabel(label), element.span.clone()))
				}
			}
			_ => (),
		}
	}
	errors
}
