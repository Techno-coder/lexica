use crate::source::Spanned;

use super::{AnnotationMap, Element, ElementParser, ParserContext, ParserError, TranslationUnit};

pub fn parse<'a>(text: &'a str, annotation_map: &'a AnnotationMap)
                 -> Result<TranslationUnit, Vec<Spanned<ParserError<'a>>>> {
	let mut errors = Vec::new();
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

//	for element in elements {
//		match element.node {
//			Element::Annotation(annotation) => unimplemented!(),
//			Element::Instruction(_) => {},
//			Element::FunctionLabel(_) => {},
//			Element::ReverseLabel(_) => {},
//			Element::LocalLabel(_) => {},
//			Element::Label(_) => {},
//			Element::ReversalHint => {},
//		}
//	}

	match errors.is_empty() {
		true => Ok(unit),
		false => Err(errors),
	}
}
