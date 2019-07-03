use crate::source::Spanned;

use super::{Annotation, Annotator, Argument, Element, ParserContext, ParserError,
            ParserResult, Size, TranslationUnit};

/// Registers locals into a function label
#[derive(Debug)]
pub struct LocalAnnotation;

impl Annotator for LocalAnnotation {
	fn arity(&self) -> usize {
		1
	}

	fn annotate<'a>(&self, annotation: &Spanned<Annotation<'a>>, context: &ParserContext,
	                unit: &mut TranslationUnit) -> ParserResult<'a, ()> {
		let arguments = &annotation.arguments;
		let size = Size::parse(argument!(arguments[0], Argument::String(size), size))
			.map_err(|error| Spanned::new(error, arguments[0].span))?;

		match context.last_element().node {
			Element::Function(_) => {
				let function = context.pending_function(unit);
				function.locals.register(size.primitive());
				Ok(())
			}
			_ => Err(Spanned::new(ParserError::InvalidApplication(annotation.identifier),
			                      annotation.span))
		}
	}
}
