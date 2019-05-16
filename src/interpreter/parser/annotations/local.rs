use crate::source::Spanned;

use super::{Annotation, Annotator, Argument, Element, ParserContext, ParserError,
            ParserResult, Size, TranslationUnit};

/// Registers locals into a function label
#[derive(Debug)]
pub struct LocalAnnotation;

impl Annotator for LocalAnnotation {
	fn arity(&self) -> usize {
		2
	}

	fn annotate<'a>(&self, annotation: &Spanned<Annotation<'a>>, context: &ParserContext,
	                unit: &mut TranslationUnit) -> ParserResult<'a, ()> {
		let arguments = &annotation.arguments;
		let size = argument!(arguments[0], Argument::String(size), size);
		let local = argument!(arguments[1], Argument::Primitive(primitive), primitive);

		let size = Size::parse(size)
			.map_err(|error| Spanned::new(error, arguments[0].span.clone()))?;
		let local = local.clone().cast(size)
			.ok_or(arguments[1].map(|node| ParserError::UnexpectedArgument(node.clone())))?;

		match context.last_element().node {
			Element::Function(_) => {
				let function = context.pending_function(unit);
				function.locals.register(local);
				Ok(())
			}
			_ => Err(Spanned::new(ParserError::InvalidApplication(annotation.identifier),
			                      annotation.span.clone()))
		}
	}
}
