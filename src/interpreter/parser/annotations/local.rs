use crate::source::Spanned;

use super::{Annotation, AnnotationType, Argument, Element, ParserContext, ParserError,
            ParserResult, Size, TranslationUnit};

#[derive(Debug)]
pub struct LocalAnnotation;

impl AnnotationType for LocalAnnotation {
	fn argument_count(&self) -> usize {
		2
	}

	fn annotate<'a>(&self, annotation: &'a Spanned<Annotation>, context: &'a ParserContext,
	                unit: &mut TranslationUnit) -> ParserResult<'a, ()> {
		let arguments = &annotation.arguments;
		let size = argument!(arguments[0], Argument::String(size), size);
		let local = argument!(arguments[1], Argument::Primitive(primitive), primitive);

		let size = Size::parse(size)
			.map_err(|error| Spanned::new(error, arguments[0].span.clone()))?;
		let local = local.clone().cast(size)
		                 .ok_or(arguments[1].map(|node| ParserError::UnexpectedArgument(node)))?;

		match context.last_element.node {
			Element::FunctionLabel(function_label) => {
				let function = unit.functions.get_mut(function_label).unwrap();
				function.locals.register(local);
				Ok(())
			}
			_ => Err(Spanned::new(ParserError::InvalidApplication(annotation.identifier), annotation.span.clone()))
		}
	}
}
